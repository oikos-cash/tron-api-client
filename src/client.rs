use reqwest::{Client as HttpClient, Method, RequestBuilder, Response};
use url::Url;
use crate::response::{NodeInfo, Block, Account, Transaction, TransactionInfo};
use serde::{de::DeserializeOwned, Serialize};
use crate::params::*;
use crate::error::{Error, Result};
use serde_json;

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    // private_key: String,
    http_client: HttpClient,
}

pub enum Address {
    Base58(String),
    Hex(String)
}
pub struct TxId(pub String);

async fn decode_response<T>(res: Response) -> Result<T>
where
    T: DeserializeOwned
{
    let data = res.text().await?;
    // dbg!(&data);

    let s: T = serde_json::from_str(&data).map_err(|orig_err| {
        match serde_json::from_str(&data) {
            Err(_) => {
                dbg!(&data);
                orig_err.into()
            },
            Ok(r) => Error::ServerError(r)
        }
    })?;

    Ok(s)
}


impl Client {
    pub fn new(base_url: String) -> Self {
        Client {
            base_url: Url::parse(&base_url).expect("could not parse base_url"),
            http_client: HttpClient::new(),
        }
    }
    // todo: for_network(shasta) -> Client (uses trongrid.io api url for shasta

    async fn post<T, U>(&self, path: &str, param: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize
    {
        let res = self
            .prep_req(Method::POST, self.get_url(path))
            .await?
            .json(&param)
            .send()
            .await?;
        decode_response::<T>(res).await
    }

    pub async fn node_info(&self) -> Result<NodeInfo> {
        let res = self
            .prep_req(Method::GET, self.node_info_url())
            .await?
            .send()
            .await?;
        decode_response::<NodeInfo>(res).await
    }

    pub async fn get_block_by_num(&self, num: u32) -> Result<Block> {
        self.post("/wallet/getblockbynum", GetBlockByNumParams::new(num)).await
    }
pub async fn get_block_by_id(&self, id: &str) -> Result<Block> {
        self.post("/wallet/getblockbyid", GetBlockByIdParams::new(id.into())).await
    }

    pub async fn get_now_block(&self) -> Result<Block> {
        self.post("/wallet/getnowblock", EmptyBody::default()).await
    }

    // TODO:
    // walletgetblockbylatestnum
    // getblockbylimitnext
    // createtransaction
    // getnowblock
    // listnodes
    // gettransactioninfobyid
    // gettransactionbyid
    // getchainparameters
    // etc...

    // TODO

    pub async fn get_account(&self, address: Address) -> Result<Account> {
        self.post("/walletsolidity/getaccount", GetAccountParams::new(address)).await
    }

    pub async fn get_transaction_by_id(&self, tx_id: TxId) -> Result<Transaction> {
        self.post("/wallet/gettransactionbyid", GetTransactionParams::new(tx_id)).await
    }

    pub async fn get_transaction_info_by_id(&self, tx_id: TxId) -> Result<TransactionInfo> {
        self.post("/wallet/gettransactioninfobyid", GetTransactionParams::new(tx_id)).await
    }

    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json");
        Ok(req)
    }

    fn node_info_url(&self) -> Url {
        self.base_url
            .join("/wallet/getnodeinfo")
            .expect("could not parse nodeinfo")
    }

    fn get_url(&self, path: &str) -> Url {
        self.base_url
            .join(path)
            .expect("could not parse url")
    }
}
