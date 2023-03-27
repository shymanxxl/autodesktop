use std::error;
use std::fmt;

pub type Result<'a, T> = std::result::Result<T, MouseError<'a>>;

#[derive(Debug, Clone)]

pub enum MouseError<'a> {
    LockXOver(&'a str),
    LockYOver(&'a str),
    LockXYOver(&'a str),
}

impl<'a> fmt::Display for MouseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::LockXOver(_) => write!(f, "x value are out of the lock range!"),
            Self::LockYOver(_) => write!(f, "y value are out of the lock range!"),
            Self::LockXYOver(_) => write!(f, "x y both are out of the lock range!"),
        }
    }
}

impl<'a> error::Error for MouseError<'a> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
