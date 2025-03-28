pub mod cli;
pub mod grid;
pub mod hash;
pub mod rand;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Solution {
    OneOutput { part_1: Answer },
    TwoOutput { part_1: Answer, part_2: Answer },
}

impl core::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneOutput { part_1 } => write!(f, "{part_1} {:21}", ""),
            Self::TwoOutput { part_1, part_2 } => write!(f, "{part_1} {part_2}"),
        }
    }
}

impl<T: Into<Answer>> From<T> for Solution {
    fn from(value: T) -> Self {
        Self::OneOutput {
            part_1: value.into(),
        }
    }
}

impl<T: Into<Answer>, J: Into<Answer>> From<(T, J)> for Solution {
    fn from(value: (T, J)) -> Self {
        Self::TwoOutput {
            part_1: value.0.into(),
            part_2: value.1.into(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Answer {
    String(String),
    Integer(u64),
}

impl From<&str> for Answer {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Answer {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u64> for Answer {
    fn from(value: u64) -> Self {
        Self::Integer(value)
    }
}

impl From<usize> for Answer {
    fn from(value: usize) -> Self {
        Self::Integer(value as u64)
    }
}

impl core::fmt::Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(string) => write!(f, "{string:<21}"),
            Self::Integer(integer) => write!(f, "{integer:<21}"),
        }
    }
}
