#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub ret: Vec<Ret>,
    pub signature: Vec<String>,
    #[serde(rename = "txID")]
    pub tx_id: String,
    #[serde(rename = "raw_data")]
    pub raw_data: RawData,
    #[serde(rename = "raw_data_hex")]
    pub raw_data_hex: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ret {
    pub contract_ret: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawData {
    pub contract: Vec<Contract>,
    #[serde(rename = "ref_block_bytes")]
    pub ref_block_bytes: String,
    #[serde(rename = "ref_block_hash")]
    pub ref_block_hash: String,
    pub expiration: i64,
    pub timestamp: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub parameter: Parameter,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub value: Value,
    #[serde(rename = "type_url")]
    pub type_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub data: Option<String>,
    #[serde(rename = "owner_address")]
    pub owner_address: String,
    #[serde(rename = "contract_address")]
    pub contract_address: Option<String>,
    #[serde(rename = "call_value")]
    pub call_value: Option<i64>,
    pub amount: Option<i64>,
    #[serde(rename = "asset_name")]
    pub asset_name: Option<String>,
    #[serde(rename = "to_address")]
    pub to_address: Option<String>,
}
