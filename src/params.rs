use crate::client::{Address, TxId};
use serde_derive::Serialize;

/// Parameters used to get series images with
/// [`Client::series_images_query`](../client/struct.Client.html#method.series_images_query).
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockByNumParams {
    num: u32,
}

impl GetBlockByNumParams {
    pub fn new(num: u32) -> GetBlockByNumParams {
        GetBlockByNumParams { num }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBlockByIdParams {
    #[serde(rename = "value")]
    id: String,
}

impl GetBlockByIdParams {
    pub fn new(id: String) -> GetBlockByIdParams {
        GetBlockByIdParams { id }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAccountParams {
    #[serde(rename = "address")]
    address: String,
    // true if address is in base58...
    visible: bool,
}

impl GetAccountParams {
    pub fn new(address: Address) -> GetAccountParams {
        let (address, visible) = match address {
            Address::Base58(addr) => (addr, true),
            Address::Hex(addr) => (addr, false),
        };

        GetAccountParams { address, visible }
    }
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionParams {
    #[serde(rename = "value")]
    id: String,
}

impl GetTransactionParams {
    pub fn new(tx_id: TxId) -> GetTransactionParams {
        GetTransactionParams { id: tx_id.0 }
    }
}
