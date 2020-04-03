#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "account_name")]
    pub account_name: String,
    pub address: String,
    pub balance: i64,
    #[serde(default)]
    pub asset: Vec<Asset>,
    #[serde(rename = "create_time", default)]
    pub create_time: i64,
    #[serde(rename = "latest_opration_time", default)]
    pub latest_opration_time: i64,
    #[serde(rename = "latest_consume_time", default)]
    pub latest_consume_time: i64,
    #[serde(rename = "latest_consume_free_time", default)]
    pub latest_consume_free_time: i64,
    #[serde(rename = "account_resource")]
    pub account_resource: AccountResource,
    #[serde(default)]
    pub asset_v2: Vec<AssetV2>,
    #[serde(rename = "free_asset_net_usageV2", default)]
    pub free_asset_net_usage_v2: Vec<FreeAssetNetUsageV2>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub key: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResource {
    #[serde(rename = "latest_consume_time_for_energy", default)]
    pub latest_consume_time_for_energy: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetV2 {
    pub key: String,
    pub value: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FreeAssetNetUsageV2 {
    pub key: String,
    pub value: i64,
}
