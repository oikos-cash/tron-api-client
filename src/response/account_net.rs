#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountNet {
    pub free_net_limit: i64,
    pub asset_net_used: Vec<AssetNetUsed>,
    pub asset_net_limit: Vec<AssetNetLimit>,
    #[serde(rename = "TotalNetLimit")]
    pub total_net_limit: i64,
    #[serde(rename = "TotalNetWeight")]
    pub total_net_weight: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetNetUsed {
    pub key: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetNetLimit {
    pub key: String,
    pub value: i64,
}
