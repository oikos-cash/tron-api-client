#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIssueList {
    pub asset_issue: Vec<AssetIssue>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetIssue {
    #[serde(rename = "owner_address")]
    pub owner_address: String,
    pub name: String,
    pub abbr: Option<String>,
    #[serde(rename = "total_supply")]
    pub total_supply: i64,
    #[serde(rename = "trx_num")]
    pub trx_num: i64,
    pub num: i64,
    #[serde(rename = "start_time")]
    pub start_time: i64,
    #[serde(rename = "end_time")]
    pub end_time: i64,
    pub description: String,
    pub url: String,
    pub id: String,
    #[serde(rename = "frozen_supply")]
    #[serde(default)]
    pub frozen_supply: Vec<FrozenSupply>,
    #[serde(rename = "public_free_asset_net_usage")]
    pub public_free_asset_net_usage: Option<i64>,
    #[serde(rename = "public_latest_free_net_time")]
    pub public_latest_free_net_time: Option<i64>,
    #[serde(rename = "vote_score")]
    pub vote_score: Option<i64>,
    #[serde(rename = "free_asset_net_limit")]
    pub free_asset_net_limit: Option<i64>,
    #[serde(rename = "public_free_asset_net_limit")]
    pub public_free_asset_net_limit: Option<i64>,
    pub precision: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FrozenSupply {
    #[serde(rename = "frozen_amount")]
    pub frozen_amount: i64,
    #[serde(rename = "frozen_days")]
    pub frozen_days: i64,
}
