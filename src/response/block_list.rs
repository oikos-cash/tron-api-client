use crate::response::Block;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockList {
    #[serde(default, rename = "block")]
    pub blocks: Vec<Block>,
}
