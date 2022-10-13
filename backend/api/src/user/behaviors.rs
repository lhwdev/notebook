use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;

use crate::common::strategy::CacheStrategy;

pub trait Logout {
    fn logout(self) -> Result<(), LogoutError>;
}

#[derive(Error, Debug)]
pub enum LogoutError {
    #[error("Already logout")]
    AlreadyLogout,

    #[error("Permission denied")]
    PermissionDenied,

    #[error(transparent)]
    Unknown(Box<dyn Error + 'static>),
}

/// What a funky name
#[async_trait]
pub trait GetMe<Me, Error> {
    fn me(&self) -> Me;

    async fn fetch_me(&mut self) -> Result<Me, Error>;

    async fn fetch_me_with(&mut self, strategy: &CacheStrategy) -> Result<Me, Error>;
}

#[async_trait]
pub trait EditMe<Me, Edit, Error> {
    async fn edit_me(edit: Edit) -> Result<Me, Error>;
}
