pub mod user;

use serde::Serialize;

#[derive(Serialize)]
pub struct CommonResponse<T> {
    pub message: Option<String>,
    pub data: T,
    pub status: u16,
}

#[derive(Serialize)]
pub struct NoResponseData;
