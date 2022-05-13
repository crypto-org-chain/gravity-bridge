use ethereum_gravity::{
    utils::GasCost,
};
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use gravity_utils::ethereum::{format_eth_address};
use gravity_utils::types::{ Erc20Token, };
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;
use gravity_utils::types::config::RelayerMode;
use reqwest::Error;

const DEFAULT_TOKEN_PRICES_PATH: &str = "token_prices.json";
const DEFAULT_TOKEN_ADDRESSES_PATH: &str = "token_addresses.json";
const DEFAULT_RELAYER_API_URL: &str = "https://cronos.org/gravity-testnet2/api/v1/oracle/query";

pub struct FeeManager {
    token_price_map: HashMap<String, String>,
    token_api_url_map: HashMap<String, String>,
    next_batch_send_time: HashMap<EthAddress, Instant>,
    mode: RelayerMode,
}

#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    status: String,
    result: ApiResult,
}

#[derive(serde::Deserialize, Debug)]
struct ApiResult {
    tokenPairDecimal: u32,
    aggregatedPriceBn: String,
}

impl FeeManager {
    pub async fn new_fee_manager(mode: RelayerMode) -> Result<FeeManager, ()> {
        let mut fm =  Self {
            token_price_map: Default::default(),
            token_api_url_map: Default::default(),
            next_batch_send_time: HashMap::new(),
            mode
        };

        fm.init().await;
        return Ok(fm);
    }

    async fn init(&mut self) -> Result<(), ()> {
        match self.mode {
            RelayerMode::Api => {
                self.initWithApi().await;
            }
            RelayerMode::File => {
                self.initWithFile().await;
            }
            RelayerMode::AlwaysRelay => {
            }
        }
        return Ok(());
    }

    async fn initWithApi(&mut self) -> Result<(), ()> {
        let token_addresses_path =
            std::env::var("TOKEN_ADDRESSES_JSON").unwrap_or_else(|_| DEFAULT_TOKEN_ADDRESSES_PATH.to_owned());

        let token_addresses_str = tokio::fs::read_to_string(token_addresses_path)
            .await
            .map_err(|e| {
                error!("Error while fetching token pair addresses {}", e);
                ()
            })?;

        let token_addresses: HashMap<String, String> = serde_json::from_str(&token_addresses_str)
            .map_err(|e| {
                error!("Error while parsing token pair addresses json configuration: {}", e);
                ()
            })?;

        let api_url =
            std::env::var("TOKEN_API_URL").unwrap_or_else(|_| DEFAULT_RELAYER_API_URL.to_owned());

        for (key, value) in token_addresses.into_iter() {
            if value == "ETH" {
                self.token_api_url_map.insert(key, String::from("ETH"));
            } else {
                let request_url = format!("{url}/{pair}",
                                          url = api_url,
                                          pair = value);

                self.token_api_url_map.insert(key, request_url);
            }
        }

        return Ok(());
    }

    async fn initWithFile(&mut self) -> Result<(), ()> {
        let config_file_path =
            std::env::var("TOKEN_PRICES_JSON").unwrap_or_else(|_| DEFAULT_TOKEN_PRICES_PATH.to_owned());

        let config_str = tokio::fs::read_to_string(config_file_path)
            .await
            .map_err(|e| {
                error!("Error while fetching token prices {}", e);
                ()
            })?;

        let config: HashMap<String, String> = serde_json::from_str(&config_str)
            .map_err(|e| {
                error!("Error while parsing token pair prices json configuration: {}", e);
                ()
            })?;

        self.token_price_map = config;
        return Ok(());
    }

    // A batch can be send either if
    // - Mode is AlwaysRelay
    // - Mode is either API or File and the batch has a profitable cost
    // - Mode is either API or File and the batch has been waiting to be sent more than GRAVITY_BATCH_SENDING_SECS secs
    pub async fn can_send_batch(
        &mut self,
        estimated_cost: &GasCost,
        batch_fee: &Erc20Token,
        contract_address: &EthAddress,
    ) -> bool {
        if self.mode == RelayerMode::AlwaysRelay {
            return true;
        }

        match self.next_batch_send_time.get(contract_address) {
            Some(time) => {
                if *time < Instant::now() {
                    return true;
                }
            }
            None => self.update_next_batch_send_time(*contract_address),
        }

        let token_price = match self.get_token_price(&batch_fee.token_contract_address).await {
            Ok(token_price) => token_price,
            Err(_) => return false,
        };

        let estimated_fee = estimated_cost.get_total();
        let batch_value = batch_fee.amount * token_price;

        info!("estimate cost is {}, batch value is {}", estimated_fee, batch_value);
        batch_value >= estimated_fee
    }

    pub fn update_next_batch_send_time(
        &mut self,
        contract_address: EthAddress,
    ) {
        if self.mode == RelayerMode::AlwaysRelay {
            return;
        }

        let timeout_duration = std::env::var("GRAVITY_BATCH_SENDING_SECS")
            .map(|value| Duration::from_secs(value.parse().unwrap()))
            .unwrap_or_else(|_| Duration::from_secs(3600));

        self.next_batch_send_time.insert(contract_address, Instant::now() + timeout_duration);
    }

    async fn get_token_price(&mut self, contract_address: &EthAddress) -> Result<U256, ()> {
        match self.mode {
            RelayerMode::Api => {
                return if let Some(request_url) = self.token_api_url_map.get(&format_eth_address(*contract_address)) {
                    // Return because gas price is quoted in ETH
                    if request_url == "ETH" {
                        return U256::from_dec_str("1").map_err(|e| {
                            error!("Cannot convert decimal to u256");
                            ()})
                    }

                    let response = reqwest::get(request_url)
                        .await.map_err(|e| error!("Cannot parse response from oracle")).unwrap();
                    let result: ApiResponse = response.json()
                        .await.map_err(|e| error!("Cannot parse result from oracle")).unwrap();
                    return U256::from_dec_str(&result.result.aggregatedPriceBn)
                        .map_err(|e| error!("Cannot convert token price"));
                } else {
                    log::error!("contract address cannot be found in token pair");
                    Err(())
                }
            }
            RelayerMode::File => {
                return if let Some(token_price_str) = self.token_price_map.get(&format_eth_address(*contract_address)) {
                    let token_price = U256::from_dec_str(token_price_str)
                        .map_err(|_| {log::error!("Unable to parse token price"); ()})?;
                    return Ok(token_price)
                } else {
                    error!("Cannot find token price in map");
                    Err(())
                }
            }
            _ => {  Err(()) }
        }
    }
}