// use chrono::{Duration, Utc};
// use lazy_static::lazy_static;
// use tokio::sync::{Mutex, MutexGuard};

use tron_api_client::{Address, Client, TxId};

// mod data;

// use data::*;

fn get_client() -> Client {
    let client = Client::for_shasta();
    client
}

fn get_client_main() -> Client {
    let client = Client::for_main();
    client
}

#[tokio::test]
async fn get_node_info() {
    let client = get_client();

    let info = client
        .get_node_info()
        .await
        .expect("Error fetching node info");
    // dbg!(info);
}

#[tokio::test]
async fn node_list() {
    let client = get_client();

    let info = client.list_nodes().await.expect("Error fetching node list");
    // dbg!(info);
}

#[tokio::test]
async fn list_witnesses() {
    let client = get_client();

    let info = client
        .list_witnesses()
        .await
        .expect("Error fetching node list");
    dbg!(info);
}

#[tokio::test]
async fn get_block_by_num() {
    let client = get_client();

    let info = client
        .get_block_by_num(10)
        .await
        .expect("Error fetching block by num");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_latest_num() {
    let client = get_client();

    let info = client
        .get_block_by_latest_num(3)
        .await
        .expect("Error fetching num latest blocks");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_limit_next() {
    let client = get_client();

    let info = client
        .get_block_by_limit_next(1_000_000, 1_000_003)
        .await
        .expect("Error fetching block by limit next");
    dbg!(info);
}

#[tokio::test]
async fn get_block_by_num_with_transactions() {
    let client = get_client();

    let info = client
        .get_block_by_num(3412121)
        .await
        .expect("Error fetching block by num");
    // dbg!(info);
}

#[tokio::test]
async fn get_block_by_id() {
    let client = get_client();

    let info = client
        .get_block_by_id("000000000000000a4efe701d7a03ff578104c6c1995ab70e713c30318b266e90")
        .await
        .expect("Error fetching block by id");
}

#[tokio::test]
async fn get_account() {
    let client = get_client();

    let info = client
        .get_account(Address::Hex(
            "41E552F6487585C2B58BC2C9BB4492BC1F17132CD0".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_2() {
    let client = get_client();

    let info = client
        .get_account(Address::Hex(
            "41a8a07f09def5e6a4462df90068c11abf6224e865".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_net() {
    let client = get_client_main();

    let info = client
        .get_account_net(Address::Hex(
            "41E552F6487585C2B58BC2C9BB4492BC1F17132CD0".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_net2() {
    let client = get_client();

    let info = client
        .get_account_net(Address::Hex(
            "41a8a07f09def5e6a4462df90068c11abf6224e865".into(),
        ))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_account_2_base58() {
    let client = get_client();

    let info = client
        .get_account(Address::Base58("TRLpnm6Uz9s2Fcy3Q235k3SiAEBXGJCNq2".into()))
        .await
        .expect("Error fetching account");
}

#[tokio::test]
async fn get_transaction_by_id() {
    let client = get_client();

    let info = client
        .get_transaction_by_id(TxId(
            "809e9d9aa5381f32f748618e4d592a58542e21fe794f35959ce811f2a58fc969".into(),
        ))
        .await
        .expect("Error fetching tx by id");
}

#[tokio::test]
async fn get_transaction_info_by_id() {
    let client = get_client();

    let info = client
        .get_transaction_info_by_id(TxId(
            "809e9d9aa5381f32f748618e4d592a58542e21fe794f35959ce811f2a58fc969".into(),
        ))
        .await
        .expect("Error fetching tx info by id");
}

#[tokio::test]
async fn get_now_block() {
    let client = get_client();

    let info = client
        .get_now_block()
        .await
        .expect("Error fetching now block");
}

#[tokio::test]
async fn get_chain_parameters() {
    let client = get_client();

    let info = client
        .get_chain_parameters()
        .await
        .expect("Error fetching chain parameters");
}

// contract 417ca2c40d9aa986b6608e07a68ebf33ea5f19a866

#[tokio::test]
async fn get_contract() {
    let client = get_client();

    let info = client
        .get_contract(Address::Hex(
            "417ca2c40d9aa986b6608e07a68ebf33ea5f19a866".into(),
        ))
        .await
        .expect("Error fetching contract");
}
