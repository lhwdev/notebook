pub mod behaviors;

use behaviors::*;
use model::user::User;
use std::error::Error;
use thiserror::Error;

pub trait UserSubject: UserBehavior + Logout {}

pub trait UserTarget: UserBehavior {}

pub trait UserBehavior: GetData<User, UserFetchError> {}

#[derive(Error, Debug)]
pub enum UserFetchError {
    #[error("Permission denied")]
    PermissionDenied,

    #[error(transparent)]
    Unknown(Box<dyn Error + 'static>),
}
