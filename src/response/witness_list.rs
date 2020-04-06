#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WitnessList {
    #[serde(default)]
    pub witnesses: Vec<Witness>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Witness {
    pub address: String,
    pub vote_count: Option<i64>,
    pub url: String,
    pub total_produced: Option<i64>,
    pub total_missed: Option<i64>,
    pub latest_block_num: Option<i64>,
    pub latest_slot_num: Option<i64>,
    pub is_jobs: Option<bool>,
}
