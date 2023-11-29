use crate::types::base::MCType;
use crate::types::boolean::MCBoolean;
use std::io::Read;

impl<P: MCType> MCType for Option<P> {
    fn pack(&self) -> Vec<u8> {
        match self {
            None => MCBoolean::from(false).pack(),
            Some(inner) => {
                let mut result = Vec::new();
                result.extend(MCBoolean::from(true).pack());
                result.extend(inner.pack());
                result
            }
        }
    }

    fn unpack(src: &mut dyn Read) -> Self {
        let is_some = MCBoolean::unpack(src);
        if is_some.into() {
            Some(P::unpack(src))
        } else {
            None
        }
    }
}
