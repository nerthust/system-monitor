use std::sync::mpsc::RecvError;

use procfs::ProcError;

#[derive(Debug)]
pub struct RTopError {
    pub err_msg: String,
}

impl From<ProcError> for RTopError {
    fn from(err: ProcError) -> Self {
        RTopError {
            err_msg: format!("{:?}", err),
        }
    }
}

impl From<std::io::Error> for RTopError {
    fn from(err: std::io::Error) -> Self {
        RTopError {
            err_msg: err.to_string(),
        }
    }
}

impl From<crossterm::ErrorKind> for RTopError {
    fn from(err: crossterm::ErrorKind) -> Self {
        RTopError {
            err_msg: err.to_string(),
        }
    }
}

impl From<RecvError> for RTopError {
    fn from(err: RecvError) -> Self {
        RTopError {
            err_msg: err.to_string(),
        }
    }
}
