#[derive(Debug)]
pub enum Error {
    Db { err: Box<dyn std::error::Error> },
    NotFound { message: &'static str },
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Db { err } => write!(f, "[DB]; {}", err),
            Self::NotFound { message } => write!(f, "[NOT FOUND] {}", message),
        }
    }
}
