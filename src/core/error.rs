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
