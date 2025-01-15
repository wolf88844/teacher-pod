pub mod account;
pub mod data;
pub mod podcast;
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiData<T: serde::Serialize> {
    pub data: T,
    pub code: u16,
    pub message: String,
}

impl<T: serde::Serialize + Default> Default for ApiData<T> {
    fn default() -> Self {
        Self {
            data: Default::default(),
            code: 400,
            message: Default::default(),
        }
    }
}
