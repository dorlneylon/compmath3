use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Request {
    pub category: usize,
    pub N: usize,
    pub eps: f32,
    pub lb: f32,
    pub rb: f32,
    pub method: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Response {
    pub x: f32,
    pub N: usize,
    pub acc: f32,
    pub errors: String,
}
