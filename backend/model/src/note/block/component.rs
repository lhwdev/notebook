use utils::note_node;

use crate::note::component;

use super::block;

#[note_node(block)]
pub struct Component {
    pub component_id: component::ComponentId,
    pub parameters: serde_json::Value,
}
