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

pub enum Extra {
    Basic(Type),
    Byte,
    Half,
}

pub enum Abi {
    Basic(Type),
    UnsignedByte,
    UnsignedHalf,
    SignedByte,
    SignedHalf,
    Name(String),
}

impl Display for Abi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Abi::Basic(t) => write!(f, "{t}"),
            Abi::UnsignedByte => write!(f, "ub"),
            Abi::UnsignedHalf => write!(f, "uh"),
            Abi::SignedByte => write!(f, "sb"),
            Abi::SignedHalf => write!(f, "sh"),
            Abi::Name(n) => write!(f, ":{n}"),
        }
    }
}
