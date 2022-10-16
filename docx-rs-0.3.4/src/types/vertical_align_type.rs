use std::fmt;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};
use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum VAlignType {
    Top,
    Center,
    Bottom,
    Unsupported,
}

impl fmt::Display for VAlignType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VAlignType::Top => write!(f, "top"),
            VAlignType::Center => write!(f, "center"),
            VAlignType::Bottom => write!(f, "bottom"),
            VAlignType::Unsupported => write!(f, "unsupported"),
        }
    }
}

impl FromStr for VAlignType {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(VAlignType::Top),
            "center" => Ok(VAlignType::Center),
            "bottom" => Ok(VAlignType::Bottom),
            _ => Ok(VAlignType::Unsupported),
        }
    }
}
