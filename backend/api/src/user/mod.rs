pub mod behaviors;

use behaviors::*;
use model::user::User;

pub trait UserSubject: UserBehavior + Logout {}

pub trait UserTarget: UserBehavior {}

pub trait UserBehavior: Me<User> {}
