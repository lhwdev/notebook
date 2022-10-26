use serde::{Deserialize, Serialize};

pub use utils_macro::note_node;

pub trait ThinWrapper {}
pub trait ThinWrapperSerde<'de>: ThinWrapper + Serialize + Deserialize<'de> {}
pub trait ThinOrmWrapper<'de>: ThinWrapperSerde<'de> {}

pub use utils_macro::ThinOrmWrapper;
pub use utils_macro::{ThinWrapper, ThinWrapperSerde};

pub use utils_macro::{deref_enum, into_enum};

pub trait GeneratePatch {}

pub use utils_macro::GeneratePatch;
