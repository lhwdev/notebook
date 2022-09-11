use sea_orm::DeriveEntityModel;
use serde::{Serialize, Deserialize};

pub type Uid = u32;

#[derive(PartialEq, Debug, Clone, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: Uid,

  pub name: String,

  pub nickname: String,

  pub password: (),

  pub hi: MyStruct
}

pub struct MyStruct {
  a: i32
}

