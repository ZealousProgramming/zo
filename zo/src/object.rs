use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Str(String),
    Null,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Number(n) => write!(f, "{n}"),
            Object::Str(s) => write!(f, "\"{s}\""),
            Object::Null => write!(f, "NULL"),
            Object::True => write!(f, "TRUE"),
            Object::False => write!(f, "FALSE"),
        }
    }
}
