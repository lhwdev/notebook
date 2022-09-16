use serde::{Deserialize, Serialize};

pub trait ThinWrapper {}
pub trait ThinWrapperSerde<'de>: ThinWrapper + Serialize + Deserialize<'de> {}
pub trait ThinOrmWrapper<'de>: ThinWrapperSerde<'de> {}

pub use utils_macro::{ThinWrapper, ThinWrapperSerde};
pub use utils_macro::ThinOrmWrapper;
