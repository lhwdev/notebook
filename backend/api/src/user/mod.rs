pub mod behaviors;

use model::user::User;
use behaviors::*;

pub trait UserSubject: UserBehavior + Logout {}

pub trait UserTarget: UserBehavior {}

pub trait UserBehavior {
    fn user() -> User;
}
