use crate::response::Transaction;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    #[serde(rename = "blockID")]
    pub block_id: String,
    #[serde(rename = "block_header")]
    pub block_header: BlockHeader,
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    #[serde(rename = "raw_data")]
    pub raw_data: RawData,
    #[serde(rename = "witness_signature")]
    pub witness_signature: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawData {
    pub number: i64,
    pub tx_trie_root: String,
    #[serde(rename = "witness_address")]
    pub witness_address: String,
    pub parent_hash: String,
    pub timestamp: i64,
}
