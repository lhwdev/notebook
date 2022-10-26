use async_trait::async_trait;
use std::error::Error;
use thiserror::Error;

use crate::common::strategy::FetchStrategy;

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

#[async_trait]
pub trait GetData<Data, Error> {
    fn data(&self) -> &Data;

    fn into_data(self) -> Data;

    async fn fetch_data(&mut self) -> Result<&Data, Error> {
        self.fetch_data_with(&FetchStrategy::default()).await
    }

    async fn fetch_data_with(&mut self, strategy: &FetchStrategy) -> Result<&Data, Error>;
}

#[async_trait]
pub trait EditEntity<Data, Patch, Error> {
    async fn edit(&self, edit: Patch) -> Result<&Data, Error>;
}

#[async_trait]
pub trait DeleteEntity<Error> {
    async fn delete(self) -> Result<(), Error>;
}
