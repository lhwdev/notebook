mod core;
mod basic;
mod layout;
mod component;
mod advanced;

pub use self::core::*;
pub use self::basic::*;
pub use self::layout::*;
pub use self::component::*;
pub use self::advanced::*;

use utils::note_node;

pub type BlockId = u32;

pub trait BlockKind {
    fn block_kind(&self) -> &dyn BlockKind;

    fn id(&self) -> BlockId {
        self.block_kind().id()
    }
}

macro_rules! block {
    (
        $( #[$attr:meta] )*
        pub struct $Name:ident {
            $( pub $field_name:ident : $FieldType:ty ),* $(,)?
        }
    ) => {
        $( #[$attr] )*
        pub struct $Name {
            pub id: crate::note::block::BlockId,
            $( pub $field_name: $FieldType, )*
        }

        impl crate::note::block::BlockKind for $Name {
            fn block_kind(&self) -> &dyn crate::note::block::BlockKind {
                self
            }

            fn id(&self) -> crate::note::block::BlockId {
                self.id
            }
        }
    };
}
pub(self) use block;

macro_rules! block_kind {
    (
        $( #[$meta:meta] )+
        pub enum $name:ident $body:tt
    ) => {
        #[::utils::deref_enum(crate::note::block::BlockKind)]
        $( #[$meta] )+
        pub enum $name $body

        impl crate::note::block::BlockKind for $name {
            fn block_kind(&self) -> &dyn crate::note::block::BlockKind {
                std::ops::Deref::deref(self).block_kind()
            }
        }
    };
}
pub(self) use block_kind;

#[note_node(block_kind)]
pub enum Block {
    Text(Text),
    // TODO
}


// Basic blocks (markdown components)

// Layout blocks

// Component


// macro_rules! block_with_content {
//     (
//         $meta:meta
//         pub struct $name:ident $body:tt
//     ) => {
//         $meta
//         pub struct $name $body
//
//         impl IntoContent for $name
//     };
// }
