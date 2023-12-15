use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum Answer {
    Number(usize),
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Answer::Number(num) => write!(f, "{num}"),
        }
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
