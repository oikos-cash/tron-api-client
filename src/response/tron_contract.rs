#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    pub bytecode: String,
    #[serde(rename = "consume_user_resource_percent")]
    pub consume_user_resource_percent: i64,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "origin_address")]
    pub origin_address: String,
    pub abi: Abi,
    #[serde(rename = "origin_energy_limit")]
    pub origin_energy_limit: i64,
    #[serde(rename = "contract_address")]
    pub contract_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Abi {
    pub entrys: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(tag = "type")]
pub enum Entry {
    #[serde(rename_all = "camelCase")]
    Function {
        // Event do not have outputs
        #[serde(default)]
        outputs: Vec<Output>,
        #[serde(default)]
        constant: bool,
        name: String,
        // state_mutability: String,
        #[serde(default)]
        inputs: Vec<Input>,
    },
    #[serde(rename_all = "camelCase")]
    Constructor {
        state_mutability: String,
        #[serde(default)]
        inputs: Vec<Input>,
    },
    Event {
        name: String,
        #[serde(default)]
        inputs: Vec<Input>,
    },
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(default)]
    pub indexed: bool,
}
