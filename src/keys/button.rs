use Button::*;

pub enum Button {
    Number(u8),
    Op(char),
}

impl Button {
    pub fn string(&self) -> String {
        match self {
            Number(n) => n.to_string(),
            Op(c) => c.to_string(),
        }
    }

    pub fn char(&self) -> char {
        match self {
            Number(n) => std::char::from_digit(*n as u32, 10).unwrap(),
            Op(c) => *c,
        }
    }
}
