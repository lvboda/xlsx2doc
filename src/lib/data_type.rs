use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Text(String),
    Image(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Field {
    pub field: String,
    pub field_type: String,
    pub size: Option<(i32, i32)>,
    pub data: Option<DataType>,
}

#[derive(Debug)]
pub enum Error {
    FetchError(String),
    ParseError(String),
    PathError(String),
    IoReadError(String),
    IoWriteError(String),
    LackInfoError(String),
}