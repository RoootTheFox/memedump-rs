use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub(crate) success: bool,
    pub(crate) message: String,
    pub(crate) data: String
}

#[derive(Serialize, Deserialize)]
pub struct DatabaseMeme {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) details: String,
    pub(crate) tags: String,
    pub(crate) thumbnail: Vec<u8>,
    pub(crate) data_type: String,
    pub(crate) data_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct Meme {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) details: String,
    pub(crate) tags: String,
    pub(crate) data: Vec<u8>, /* raw data */
    pub(crate) data_type: String,
    pub(crate) data_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMeme {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) details: String,
    pub(crate) tags: Vec<String>,
    pub(crate) data: String, /* base64 */
    pub(crate) data_type: String,
    pub(crate) data_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct IndexResultMeme {
    pub(crate) id: i32,
    pub(crate) title: String,
    pub(crate) details: String,
    pub(crate) tags: Vec<String>,
    pub(crate) thumbnail: String,
    pub(crate) data_type: String,
    pub(crate) data_size: i32
}

#[derive(Serialize, Deserialize)]
pub struct UploadData {
    pub title: String,
    pub details: String,
    pub tags: Vec<String>,
    pub data: String, /* base64 */
    pub datatype: String
}