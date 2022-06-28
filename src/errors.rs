use fltk::prelude::FltkError;
use std::fmt;

pub type Result<T> = std::result::Result<T, MinesweeperError>;

#[derive(Debug)]
pub enum MinesweeperError {
    FltkErr(FltkError),
}

impl fmt::Display for MinesweeperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MinesweeperError::FltkErr(error) => {
                write!(f, "Fltk error: ")?;
                error.fmt(f)
            }
        }
    }
}
