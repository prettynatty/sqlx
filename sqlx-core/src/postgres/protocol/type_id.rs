use std::fmt::{self, Display};

#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub struct TypeId(pub(crate) u32);

impl TypeId {
    pub(crate) const BOOL: TypeId = TypeId(16);

    pub(crate) const INT2: TypeId = TypeId(21);
    pub(crate) const INT4: TypeId = TypeId(23);
    pub(crate) const INT8: TypeId = TypeId(20);
}

impl Display for TypeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            TypeId::BOOL => "BOOL",

            _ => {
                return write!(f, "{}", self.0);
            }
        };

        write!(f, "{}", name)
    }
}
