#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInfo {
    pub id: String,
    pub fee: Option<i64>,
    pub block_number: i64,
    pub block_time_stamp: i64,
    pub contract_result: Vec<String>,
    pub receipt: Receipt,
    #[serde(rename = "contract_address")]
    pub contract_address: Option<String>,
    pub log: Option<Vec<Log>>,
    #[serde(rename = "internal_transactions")]
    pub internal_transactions: Option<Vec<InternalTransaction>>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Receipt {
    #[serde(rename = "net_fee")]
    pub net_fee: Option<i64>,
    #[serde(rename = "energy_usage")]
    pub energy_usage: Option<i64>,
    #[serde(rename = "energy_usage_total")]
    pub energy_usage_total: Option<i64>,
    #[serde(rename = "net_usage")]
    pub net_usage: Option<i64>,
    pub result: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: String,
    pub topics: Vec<String>,
    pub data: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalTransaction {
    pub hash: String,
    #[serde(rename = "caller_address")]
    pub caller_address: String,
    #[serde(rename = "transferTo_address")]
    pub transfer_to_address: String,
    pub call_value_info: Vec<CallValueInfo>,
    pub note: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CallValueInfo {}
