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
