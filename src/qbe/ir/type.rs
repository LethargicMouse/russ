use std::fmt::Display;

pub enum Type {
    Word,
    Long,
    Float,
    Double,
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Word => write!(f, "w"),
            Type::Long => write!(f, "l"),
            Type::Float => write!(f, "s"),
            Type::Double => write!(f, "d"),
        }
    }
}

pub enum ExtraType {
    Basic(Type),
    Byte,
    Half,
}

pub enum AbiType {
    Basic(Type),
    UnsignedByte,
    UnsignedHalf,
    SignedByte,
    SignedHalf,
    Name(String),
}

impl AbiType {
    pub const WORD: Self = Self::Basic(Type::Word);
}

impl Display for AbiType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AbiType::Basic(t) => write!(f, "{t}"),
            AbiType::UnsignedByte => write!(f, "ub"),
            AbiType::UnsignedHalf => write!(f, "uh"),
            AbiType::SignedByte => write!(f, "sb"),
            AbiType::SignedHalf => write!(f, "sh"),
            AbiType::Name(n) => write!(f, ":{n}"),
        }
    }
}
