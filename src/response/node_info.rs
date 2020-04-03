use std::collections::HashMap;

// generated with https://transform.tools/json-to-rust-serde
//

/// Node info data returned by [`Client::node_info`].
///
/// See [`Client::node_info`] for more info.
///
/// [`Client::node_info`]: ../client/struct.Client.html#method.node_info
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeInfo {
    pub active_connect_count: i64,
    pub begin_sync_num: i64,
    pub block: String,
    pub cheat_witness_info_map: CheatWitnessInfoMap,
    pub config_node_info: ConfigNodeInfo,
    pub current_connect_count: i64,
    pub machine_info: MachineInfo,
    pub passive_connect_count: i64,
    pub peer_list: Vec<PeerList>,
    pub solidity_block: String,
    pub total_flow: i64,
}

type CheatWitnessInfoMap = HashMap<String, String>;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigNodeInfo {
    pub active_node_size: i64,
    pub allow_adaptive_energy: i64,
    pub allow_creation_of_contracts: i64,
    pub backup_listen_port: i64,
    pub backup_member_size: i64,
    pub backup_priority: i64,
    pub code_version: String,
    pub db_version: i64,
    pub discover_enable: bool,
    pub listen_port: i64,
    pub max_connect_count: i64,
    pub max_time_ratio: f64,
    pub min_participation_rate: i64,
    pub min_time_ratio: f64,
    #[serde(rename = "p2pVersion")]
    pub p2_p_version: String,
    pub passive_node_size: i64,
    pub same_ip_max_connect_count: i64,
    pub send_node_size: i64,
    pub support_constant: bool,
    pub version_name: String,
    pub version_num: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineInfo {
    pub cpu_count: i64,
    pub cpu_rate: f64,
    pub dead_lock_thread_count: i64,
    pub dead_lock_thread_info_list: Vec<::serde_json::Value>,
    pub free_memory: i64,
    pub java_version: String,
    pub jvm_free_memory: i64,
    pub jvm_total_memoery: i64,
    pub memory_desc_info_list: Vec<MemoryDescInfoList>,
    pub os_name: String,
    pub process_cpu_rate: f64,
    pub thread_count: i64,
    pub total_memory: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryDescInfoList {
    pub init_size: i64,
    pub max_size: i64,
    pub name: String,
    pub use_rate: f64,
    pub use_size: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeerList {
    pub active: bool,
    pub avg_latency: f64,
    pub block_in_porc_size: i64,
    pub connect_time: i64,
    pub disconnect_times: i64,
    pub head_block_time_we_both_have: i64,
    pub head_block_we_both_have: String,
    pub host: String,
    pub in_flow: i64,
    pub last_block_update_time: i64,
    pub last_sync_block: String,
    pub local_disconnect_reason: String,
    pub need_sync_from_peer: bool,
    pub need_sync_from_us: bool,
    pub node_count: i64,
    pub node_id: String,
    pub port: i64,
    pub remain_num: i64,
    pub remote_disconnect_reason: String,
    pub score: i64,
    pub sync_block_requested_size: i64,
    pub sync_flag: bool,
    pub sync_to_fetch_size: i64,
    pub sync_to_fetch_size_peek_num: i64,
    pub un_fetch_syn_num: i64,
}
