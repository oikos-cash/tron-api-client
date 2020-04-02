use reqwest::{header::HeaderValue, Client as HttpClient, Method, RequestBuilder, Response};
use url::Url;
use crate::error::{Error, Result};
use crate::response::{NodeInfo};

#[derive(Debug)]
pub struct Client {
    base_url: Url,
    // private_key: String,
    http_client: HttpClient,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Client {
            base_url: Url::parse(&base_url).expect("could not parse base_url"),
            http_client: HttpClient::new(),
        }
    }
    // todo: for_network(shasta) -> Client (uses trongrid.io api url for shasta

    pub async fn node_info(&self) -> Result<NodeInfo> {
        let res = self
            .prep_req(Method::GET, self.node_info_url())
            .await?
            .send()
            .await?;

        // api_errors(&res)?;

        Ok(res.json::<NodeInfo>().await?)
    }

    /*
    pub async fn series_into<T, I>(&self, id: I) -> Result<T>
    where
        I: Into<SeriesID>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.series_url(id.into()))
            .await?
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<T>>().await?.data)
    }
    */


    async fn prep_req(&self, method: Method, url: Url) -> Result<RequestBuilder> {
        let req = self
            .http_client
            .request(method, url)
            .header("Content-Type", "application/json");
            /*
            .bearer_auth(
                &self
                    .token
                    .lock()
                    .await
                    .as_ref()
                    .expect("missing token although ensured valid")
                    .token,
            );
            */

        Ok(req)
    }

    /*
    fn login_url(&self) -> Url {
        self.base_url
            .join("/login")
            .expect("could not parse login url")
    }
    */

    fn node_info_url(&self) -> Url {
        self.base_url
            .join("/wallet/getnodeinfo")
            .expect("could not parse nodeinfo")
    }

    /*
    fn series_url(&self, id: SeriesID) -> Url {
        self.base_url
            .join(&format!("/series/{}", id))
            .expect("could not parse series url")
    }
    */
}

/*
#[cfg(test)]
mod tests;
*/
