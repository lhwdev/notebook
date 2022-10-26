use utils::GeneratePatch;

use crate::service::ServiceReferences;

#[derive(PartialEq, Debug, Clone, GeneratePatch)]
#[generate_patch(name = "UserPatch")]
pub struct User {
    pub id: Uid,

    pub name: String,

    pub password: Password,

    #[patch(PersonalInfoPatch)]
    pub info: PersonalInfo,

    pub services: ServiceReferences,
}

#[derive(PartialEq, Debug, Clone, GeneratePatch)]
#[generate_patch(name = "PersonalInfoPatch")]
pub struct PersonalInfo {
    pub nickname: String,

    pub email: String,
}

pub type Uid = u32;

#[derive(PartialEq, Debug, Clone)]
pub struct Password(String);

pub mod orm {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    use super::Uid;

    #[derive(PartialEq, Debug, Clone, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uid,

        pub name: String,

        pub password: String,

        pub info: PersonalInfo,
    }

    #[derive(PartialEq, Debug, Clone, Serialize, Deserialize, FromJsonQueryResult)]
    pub struct PersonalInfo {
        pub nickname: String,

        pub email: String,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}
