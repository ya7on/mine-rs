use darling::FromDeriveInput;
use proc_macro::TokenStream;
use syn::__private::quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput, Default)]
#[darling(attributes(packet))]
struct Opts {
    packet_id: i32,
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
            body.extend(MCType::pack(&self.#name));
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
            fn packet_id(&self) -> i32 {
                #packet_id
            }

            fn pack(&self) -> Vec<u8> {
                let mut body = Vec::new();
                body.extend(MCVarInt::from(self.packet_id()).pack());
                #(#fields_pack)*
                let mut result = Vec::new();
                result.extend(MCVarInt::from(body.len() as i32).pack());
                result.extend(body);
                result
            }

            fn unpack(src: &mut dyn std::io::Read) -> Self {
                Self {
                    #(#fields_unpack)*
                }
            }
        }
    };

    expanded.into()
}
