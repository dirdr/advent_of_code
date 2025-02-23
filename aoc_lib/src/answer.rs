use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Answer {
    Number(usize),
    String(String),
    Unimplemented,
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::Number(num) => write!(f, "{}", num),
            Answer::String(string) => write!(f, "{}", string),
            Answer::Unimplemented => write!(f, "The answer is not implemented"),
        }
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

macro_rules! answer_impl {
    ($($answer:ident, $answer_type:ty => { $($type:ty),* }),*) => {
        $($(impl From<$type> for Answer {
            fn from(n: $type) -> Self {
                Self::$answer(n as $answer_type)
            }
        })*)*
    };
}

answer_impl!(
    Number, usize => { u8, u16, u32, u64, usize, i8, i16, i32, i64, isize }
);
