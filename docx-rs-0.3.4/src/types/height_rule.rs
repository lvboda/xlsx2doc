use std::fmt;
use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};

use super::errors;
use std::str::FromStr;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HeightRule {
    Auto,
    AtLeast,
    Exact,
}

impl Default for HeightRule {
    fn default() -> Self {
        Self::AtLeast
    }
}

impl fmt::Display for HeightRule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HeightRule::Auto => write!(f, "auto"),
            HeightRule::AtLeast => write!(f, "atLeast"),
            HeightRule::Exact => write!(f, "exact"),
        }
    }
}

impl FromStr for HeightRule {
    type Err = errors::TypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(HeightRule::Auto),
            "atLeast" => Ok(HeightRule::AtLeast),
            "exact" => Ok(HeightRule::Exact),
            _ => Ok(HeightRule::AtLeast),
        }
    }
}
