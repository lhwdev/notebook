use utils::ThinOrmWrapper;

pub type User = orm::Model;

pub type PersonalInfo = orm::PersonalInfo;

pub type Uid = u32;

#[derive(ThinOrmWrapper, PartialEq, Debug, Clone)]
pub struct Password(String);

pub mod orm {
    use sea_orm::entity::prelude::*;
    use serde::{Deserialize, Serialize};

    use super::{Uid, Password};

    #[derive(PartialEq, Debug, Clone, DeriveEntityModel, Serialize, Deserialize)]
    #[sea_orm(table_name = "users")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub id: Uid,

        pub name: String,

        pub password: Password,

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
