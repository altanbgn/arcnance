pub mod user;

#[derive(serde::Serialize)]
pub struct CommonResponse<T> {
    pub message: Option<String>,
    pub data: T,
    pub status: u16,
}

#[derive(serde::Serialize)]
pub struct NoResponseData;
