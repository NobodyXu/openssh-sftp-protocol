pub(crate) use core::fmt;

pub(crate) use serde::de::{Deserializer, SeqAccess, Visitor};
pub(crate) use serde::Deserialize;

macro_rules! impl_visitor {
    ($type:ident, $visitor_name: ident, $expecting_msg:expr, $seq_name: ident, $impl:block) => {
        impl<'de> crate::visitor::Deserialize<'de> for $type {
            fn deserialize<D: crate::visitor::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<Self, D::Error> {
                struct $visitor_name;

                impl<'de> crate::visitor::Visitor<'de> for $visitor_name {
                    type Value = $type;

                    fn expecting(
                        &self,
                        formatter: &mut crate::visitor::fmt::Formatter,
                    ) -> crate::visitor::fmt::Result {
                        write!(formatter, $expecting_msg)
                    }

                    fn visit_seq<V>(self, $seq_name: V) -> Result<Self::Value, V::Error>
                    where
                        V: crate::visitor::SeqAccess<'de>,
                    {
                        $impl
                    }
                }

                deserializer.deserialize_seq($visitor_name)
            }
        }
    };
}

pub(crate) use impl_visitor;
