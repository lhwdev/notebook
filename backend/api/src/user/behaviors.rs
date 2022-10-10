use std::error::Error;
use thiserror::Error;

pub trait Logout {
    fn logout(self) -> Result<(), LogoutError>;
}

#[derive(Error, Debug)]
pub enum LogoutError {
    #[error("Unknown error: {0}")]
    Unknown(Box<dyn Error + 'static>),
}
