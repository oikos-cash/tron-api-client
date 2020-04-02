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

    /// Search for series providing either a (partial) name, IMDb id, slug or
    /// Zap2it id.
    ///
    /// Sends a `GET` request to the `/search/series` API endpoint.
    ///
    /// # Examples
    /// ```no_run
    /// # use thetvdb::{Client, error::Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// use thetvdb::params::SearchBy;
    ///
    /// let results = client.search(SearchBy::IMDbID("tt5491994")).await?;
    ///
    /// assert_eq!(
    ///     results[0].series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    /*
    pub async fn search<S>(&self, param: SearchBy<S>) -> Result<Vec<SearchSeries>>
    where
        S: AsRef<str>,
    {
        self.search_into(param).await
    }

    /// Same as [`search`], but allows deserializing the response search series
    /// data into a provided type.
    ///
    /// [`search`]: #method.search
    pub async fn search_into<T, S>(&self, param: SearchBy<S>) -> Result<Vec<T>>
    where
        S: AsRef<str>,
        T: DeserializeOwned,
    {
        let res = self
            .prep_lang_req(Method::GET, self.search_url())
            .await?
            .query(&param.query_param())
            .send()
            .await?;

        api_errors(&res)?;

        Ok(res.json::<ResponseData<Vec<T>>>().await?.data)
    }
    */

    /// Get a series by its id.
    ///
    /// Sends a `GET` request to the `/series/{id}` API endpoint.
    ///
    /// References to `SearchSeries`, `Series`, `SeriesUpdate` or any type that
    /// impls `Into<SeriesID>` can also be used for ids.
    ///
    /// # Examples
    /// Use a literal id:
    /// ```no_run
    /// # use thetvdb::{Client, error::Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// let series = client.series(318408).await?;
    ///
    /// assert_eq!(
    ///     series.series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// Use a search result:
    /// ```no_run
    /// # use thetvdb::{Client, error::Result, params::SearchBy};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #
    /// # let client = Client::new("KEY").await?;
    /// #
    /// let results = client.search(SearchBy::IMDbID("tt5491994")).await?;
    ///
    /// let series = client.series(&results[0]).await?;
    ///
    /// assert_eq!(
    ///     series.series_name,
    ///     Some("Planet Earth II".to_string())
    /// );
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    /// Will return an error if the series is not found.
    /*
    pub async fn series<I>(&self, id: I) -> Result<Series>
    where
        I: Into<SeriesID>,
    {
        self.series_into(id).await
    }
    */

    /// Same as [`series`], but allows deserializing the response series data
    /// into a provided type.
    ///
    /// [`series`]: #method.series
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

#[cfg(test)]
mod tests;
