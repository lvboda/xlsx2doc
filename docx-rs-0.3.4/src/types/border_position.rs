use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableBorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    InsideH,
    InsideV,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableCellBorderPosition {
    Left,
    Right,
    Top,
    Bottom,
    InsideH,
    InsideV,
    Tl2br,
    Tr2bl,
}
