use crate::error::{Error, Result};
use crate::params::*;
use crate::response::{
    Account, Block, ChainParameters, Contract, NodeInfo, NodeList, Transaction, TransactionInfo,
};
use reqwest::{Client as HttpClient, Method, RequestBuilder, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json;
use url::Url;

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    // private_key: String,
    http_client: HttpClient,
}

pub enum Address {
    Base58(String),
    Hex(String),
}
pub struct TxId(pub String);

async fn decode_response<T>(res: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    let data = res.text().await?;
    // dbg!(&data);

    let s: T =
        serde_json::from_str(&data).map_err(|orig_err| match serde_json::from_str(&data) {
            Err(_) => {
                println!("{}", data);
                // dbg!(&data);
                orig_err.into()
            }
            Ok(r) => Error::ServerError(r),
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

    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json");
        Ok(req)
    }

    fn get_url(&self, path: &str) -> Url {
        self.base_url.join(path).expect("could not parse url")
    }

    async fn req<T, U>(&self, path: &str, method: Method, body: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        let res = match method {
            Method::GET => {
                self.prep_req(method, self.get_url(path))
                    .await?
                    .send()
                    .await?
            }
            Method::POST => {
                self.prep_req(method, self.get_url(path))
                    .await?
                    .json(&body)
                    .send()
                    .await?
            }
            _ => unimplemented!(),
        };
        decode_response::<T>(res).await
    }

    async fn post<T, U>(&self, path: &str, param: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize,
    {
        self.req(path, Method::POST, param).await
    }

    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.req(path, Method::GET, EmptyBody::default()).await
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        self.get("/wallet/getnodeinfo").await
    }

    pub async fn list_nodes(&self) -> Result<NodeList> {
        self.get("/wallet/listnodes").await
    }

    pub async fn get_chain_parameters(&self) -> Result<ChainParameters> {
        self.get("/wallet/getchainparameters").await
    }

    pub async fn get_block_by_num(&self, num: u32) -> Result<Block> {
        self.post("/wallet/getblockbynum", GetBlockByNumParams::new(num))
            .await
    }

    pub async fn get_block_by_id(&self, id: &str) -> Result<Block> {
        self.post("/wallet/getblockbyid", GetBlockByIdParams::new(id.into()))
            .await
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
        self.post("/walletsolidity/getaccount", GetAccountParams::new(address))
            .await
    }

    // TODO: retry if tron node returns an empty object `{}`?
    // This happens when querying a TX in a recently mined block.
    pub async fn get_transaction_by_id(&self, tx_id: TxId) -> Result<Transaction> {
        self.post(
            "/wallet/gettransactionbyid",
            GetTransactionParams::new(tx_id),
        )
        .await
    }

    pub async fn get_transaction_info_by_id(&self, tx_id: TxId) -> Result<TransactionInfo> {
        self.post(
            "/wallet/gettransactioninfobyid",
            GetTransactionParams::new(tx_id),
        )
        .await
    }

    pub async fn get_contract(&self, address: Address) -> Result<Contract> {
        self.post("/wallet/getcontract", GetContractParams::new(address))
            .await
    }
}
