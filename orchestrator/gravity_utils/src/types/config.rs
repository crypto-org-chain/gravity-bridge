use serde_derive::Deserialize;
use strum_macros::EnumString;

/// The various possible modes for relaying
#[derive(Debug, Deserialize, PartialEq, Copy, Clone, EnumString)]
pub enum RelayerMode {
    /// Always relay batches, profitable or not
    AlwaysRelay,
    /// Use private API to fetch the price data feed for the cost estimation
    Api,
    /// Use file to fetch the token price for the cost estimation
    File,
}
