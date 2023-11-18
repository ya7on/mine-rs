use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(attributes(packet))]
struct Opts {
    packet_id: u8,
}

#[proc_macro_derive(MCPacket, attributes(packet))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = Opts::from_derive_input(&input).unwrap();
    let DeriveInput { ident, data, .. } = input;

    let packet_id = opts.packet_id;

    let data = if let syn::Data::Struct(data) = data {
        data
    } else {
        unimplemented!()
    };

    let fields_pack = data.fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            result.extend(MCType::pack(&self.#name));
        }
    });

    let fields_unpack = data.fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            #name: MCType::unpack(src),
        }
    });

    let expanded = quote! {
        impl MCPacket for #ident {
            fn packet_id(&self) -> u8 {
                #packet_id
            }

            fn pack(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #(#fields_pack)*
                result
            }

            fn unpack(src: &mut Vec<u8>) -> Self {
                Self {
                    #(#fields_unpack)*
                }
            }
        }
    };

    expanded.into()
}
