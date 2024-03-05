use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Request {
    pub category: usize,
    pub eps: f32,
    pub lb: f32,
    pub rb: f32,
    pub method: usize,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub(crate) struct Response {
    pub x: (f32, f32),
    pub acc: Vec<f32>,
    pub iters: usize,
    pub errors: String,
}
