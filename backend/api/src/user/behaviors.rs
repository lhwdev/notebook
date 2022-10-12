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

    #[error("Unknown error: {0}")]
    Unknown(#[source] Box<dyn Error + 'static>),
}

#[async_trait]
pub trait Me<T> {
    fn me(&self) -> T;

    async fn fetch_me(&mut self) -> T;

    async fn fetch_me_with(&mut self, strategy: CacheStrategy) -> T;
}
