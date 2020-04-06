use clap::*;
use std::process;

use serde_json::json;
use tron_api_client::*;

// pub async fn get_node_info(&self) -> Result<NodeInfo> {
// pub async fn list_nodes(&self) -> Result<NodeList> {
// pub async fn get_chain_parameters(&self) -> Result<ChainParameters> {
// pub async fn get_block_by_num(&self, num: u32) -> Result<Block> {
// pub async fn get_block_by_id(&self, id: &str) -> Result<Block> {
// pub async fn get_now_block(&self) -> Result<Block> {
// pub async fn get_account(&self, address: Address) -> Result<Account> {
// pub async fn get_transaction_by_id(&self, tx_id: TxId) -> Result<Transaction> {
// pub async fn get_transaction_info_by_id(&self, tx_id: TxId) -> Result<TransactionInfo> {
// pub async fn get_contract(&self, address: Address) -> Result<Contract> {

// get_node_info
// list_nodes
// get_chain_parameters
// get_block_by_num num
// get_block_by_id id
// get_now_block
// get_account address: Address
// get_transaction_by_id tx_id: TxId
// get_transaction_info_by_id tx_id: TxId
// get_contract address: Address

// TODO: support base58 addresses

#[tokio::main]
async fn main() {
    let app = clap_app!(myapp =>
        (name: "tron")
        (version: crate_version!())
        // (author: crate_authors!())
        (about: crate_description!())
        (@arg network:
            --network
             default_value("main")
             possible_value[main]
             possible_value[shasta]
             env("TRON_NETWORK")
            +takes_value
            "Specify tron network (uses trongrid.io)")
        (@subcommand get_node_info =>
            (about: "Get Node Info")
        )
        (@subcommand list_nodes =>
            (about: "List Nodes")
        )
        (@subcommand get_chain_parameters =>
            (about: "Get Chain Parameters")
        )
        (@subcommand get_block_by_num =>
            (about: "Get Block by Number")
            (@arg num: +required "Block Number")
        )
        (@subcommand get_block_by_latest_num =>
            (about: "Get <num> Latest Blocks")
            (@arg num: +required "Number of blocks to fetch")
        )
        (@subcommand get_block_by_limit_next =>
            (about: "Get Range of Blocks")
            (@arg start: +required "Start of range (block height)")
            (@arg end: +required "End of range (block height)")
        )
        (@subcommand get_block_by_id =>
            (about: "Get Block by Id")
            (@arg id: +required "Block Id")
        )
        (@subcommand get_now_block =>
            (about: "Get Latest Block")
        )
        (@subcommand get_account =>
            (about: "Get Account")
            (@arg address: +required "Address (hex format)")
        )
        (@subcommand get_account_net =>
            (about: "Get Account Bandwidth")
            (@arg address: +required "Address (hex format)")
        )
        (@subcommand get_transaction_by_id =>
            (about: "Get Transaction by Id")
            (@arg id: +required "Transaction ID")
        )
        (@subcommand get_transaction_info_by_id =>
            (about: "Like get_transaction_by_id but more detailed")
            (@arg id: +required "Transaction ID")
        )
        (@subcommand get_contract =>
            (about: "Get Contract")
            (@arg address: +required "Contract Address (hex format)")
        )
    )
    .setting(clap::AppSettings::SubcommandRequiredElseHelp);

    let matches = app.get_matches();

    let (command_name, submatches) = matches.subcommand();

    // TODO: configurable!
    let client = match matches.value_of("network").unwrap() {
        "main" => Client::for_main(),
        "shasta" => Client::for_shasta(),
        _ => unimplemented!(),
    };

    match command_name {
        "get_node_info" => {
            // TODO: handle errors
            let res = client.get_node_info().await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "list_nodes" => {
            // TODO: handle errors
            let res = client.list_nodes().await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_chain_parameters" => {
            // TODO: handle errors
            let res = client.get_chain_parameters().await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_block_by_num" => {
            let submatches = submatches.unwrap();
            let num: u64 = value_t!(submatches, "num", u64).unwrap_or_else(|e| e.exit());
            let res = client.get_block_by_num(num).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_block_by_latest_num" => {
            let submatches = submatches.unwrap();
            let num: u64 = value_t!(submatches, "num", u64).unwrap_or_else(|e| e.exit());
            let res = client.get_block_by_latest_num(num).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_block_by_limit_next" => {
            let submatches = submatches.unwrap();
            let start: u64 = value_t!(submatches, "start", u64).unwrap_or_else(|e| e.exit());
            let end: u64 = value_t!(submatches, "end", u64).unwrap_or_else(|e| e.exit());
            let res = client.get_block_by_limit_next(start, end).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_block_by_id" => {
            let submatches = submatches.unwrap();
            let id: String = value_t!(submatches, "id", String).unwrap_or_else(|e| e.exit());
            let res = client.get_block_by_id(&id).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_now_block" => {
            let res = client.get_now_block().await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_account" => {
            let submatches = submatches.unwrap();
            let address: String =
                value_t!(submatches, "address", String).unwrap_or_else(|e| e.exit());
            let address = Address::Hex(address);
            let res = client.get_account(address).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_account_net" => {
            let submatches = submatches.unwrap();
            let address: String =
                value_t!(submatches, "address", String).unwrap_or_else(|e| e.exit());
            let address = Address::Hex(address);
            let res = client.get_account_net(address).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_transaction_by_id" => {
            let submatches = submatches.unwrap();
            let id: String = value_t!(submatches, "id", String).unwrap_or_else(|e| e.exit());
            let id = TxId(id);
            let res = client.get_transaction_by_id(id).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_transaction_info_by_id" => {
            let submatches = submatches.unwrap();
            let id: String = value_t!(submatches, "id", String).unwrap_or_else(|e| e.exit());
            let id = TxId(id);
            let res = client.get_transaction_info_by_id(id).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        "get_contract" => {
            let submatches = submatches.unwrap();
            let address: String =
                value_t!(submatches, "address", String).unwrap_or_else(|e| e.exit());
            let address = Address::Hex(address);
            let res = client.get_contract(address).await.unwrap();
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        _ => unimplemented!(),
    }
}

/*
fn validate_hex(hex: String) -> std::result::Result<(), String> {
    if hex.len() != 64 {
        return Err("must be 64 characters hexadecimal".to_string());
    }
    match hex::decode(hex) {
        Ok(_) => Ok(()),
        _ => Err("must be valid hexadecimal".to_string()),
    }
}
*/

fn die(msg: &str) -> ! {
    eprintln!("{}", msg);
    process::exit(1);
}
