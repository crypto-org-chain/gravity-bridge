use serde_derive::Deserialize;
use std::fmt;
use std::str::FromStr;

/// The various possible modes for relaying
#[derive(Debug, Deserialize, PartialEq, Copy, Clone)]
pub enum RelayerMode {
    /// Always relay batches, profitable or not
    AlwaysRelay,
    /// Use private API to fetch the price data feed for the cost estimation
    Api,
    /// Use file to fetch the token price for the cost estimation
    File,
}

impl FromStr for RelayerMode {
    type Err = ();
    fn from_str(input: &str) -> Result<RelayerMode, Self::Err> {
        let mode = RelayerMode::from_str(input).unwrap();
        return Ok(mode);
    }
}