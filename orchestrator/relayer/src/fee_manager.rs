use ethereum_gravity::utils::GasCost;
use ethers::prelude::*;
use ethers::types::Address as EthAddress;
use gravity_utils::ethereum::format_eth_address;
use gravity_utils::types::config::RelayerMode;
use gravity_utils::types::Erc20Token;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use std::time::Instant;

const DEFAULT_TOKEN_PRICES_PATH: &str = "token_prices.json";
const DEFAULT_RELAYER_API_URL: &str = "";

pub struct FeeManager {
    token_price_map: HashMap<String, String>,
    relayer_api_url: String,
    next_batch_send_time: HashMap<EthAddress, Instant>,
    mode: RelayerMode,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiResponse {
    can_send: bool,
    reason_type: u32,
}

impl FeeManager {
    pub async fn new_fee_manager(mode: RelayerMode) -> Result<FeeManager, ()> {
        let mut fm = Self {
            token_price_map: Default::default(),
            relayer_api_url: String::default(),
            next_batch_send_time: HashMap::new(),
            mode,
        };
        fm.init().await?;
        Ok(fm)
    }

    async fn init(&mut self) -> Result<(), ()> {
        match self.mode {
            RelayerMode::Api => {
                self.init_with_api();
            }
            RelayerMode::File => {
                self.init_with_file().await?;
            }
            RelayerMode::AlwaysRelay => {}
        }
        Ok(())
    }

    fn init_with_api(&mut self) {
        self.relayer_api_url =
            std::env::var("RELAYER_API_URL").unwrap_or_else(|_| DEFAULT_RELAYER_API_URL.to_owned());
    }

    async fn init_with_file(&mut self) -> Result<(), ()> {
        let config_file_path = std::env::var("TOKEN_PRICES_JSON")
            .unwrap_or_else(|_| DEFAULT_TOKEN_PRICES_PATH.to_owned());

        let config_str = tokio::fs::read_to_string(config_file_path)
            .await
            .map_err(|e| {
                error!("Error while fetching token prices {}", e);
            })?;

        let config: HashMap<String, String> = serde_json::from_str(&config_str).map_err(|e| {
            error!(
                "Error while parsing token pair prices json configuration: {}",
                e
            );
        })?;

        self.token_price_map = config;
        Ok(())
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
        match self.mode {
            RelayerMode::AlwaysRelay => true,
            RelayerMode::File => {
                if self.should_send_at_non_profitable_cost(contract_address) {
                    return true;
                }
                let token_price = match self
                    .get_token_price(&batch_fee.token_contract_address)
                    .await
                {
                    Ok(token_price) => token_price,
                    Err(_) => return false,
                };

                let estimated_fee = estimated_cost.get_total();
                let batch_value = batch_fee
                    .amount
                    .checked_mul(token_price)
                    .unwrap_or_else(|| {
                        error!("estimate cost value exceeded");
                        U256::from(0)
                    });
                info!(
                    "estimate cost is {}, batch value is {}",
                    estimated_fee, batch_value
                );
                batch_value >= estimated_fee
            }
            RelayerMode::Api => {
                let body = json!({
                    "batchFee": {
                        "amount": batch_fee.amount.to_string(),
                        "tokenContractAddress": batch_fee.token_contract_address
                    },
                    "estimatedCost": {
                        "gas": estimated_cost.gas.to_string(),
                        "gasPrice": estimated_cost.gas_price.to_string()
                    }}
                );

                return match Client::new()
                    .post(self.relayer_api_url.as_str())
                    .json(&body)
                    .send()
                    .await
                {
                    Ok(resp) => match resp.json().await {
                        Ok(json) => {
                            let api_response: ApiResponse = json;
                            return if api_response.can_send {
                                true
                            } else {
                                // code 5 means that it is not profitable but limit has not been
                                // exceeded or no addresses are blacklisted
                                // in that case we check if we should send at non profitable cost
                                api_response.reason_type == 5
                                    && self.should_send_at_non_profitable_cost(contract_address)
                            };
                        }
                        Err(err) => {
                            error!("error deserializing response from relayer api: {}", err);
                            false
                        }
                    },
                    Err(err) => {
                        error!("error getting response from relayer api: {}", err);
                        false
                    }
                };
            }
        }
    }

    fn should_send_at_non_profitable_cost(&mut self, contract_address: &EthAddress) -> bool {
        match self.next_batch_send_time.get(contract_address) {
            Some(time) => return *time < Instant::now(),
            None => self.update_next_batch_send_time(*contract_address),
        }
        true
    }

    pub(crate) fn update_next_batch_send_time(&mut self, contract_address: EthAddress) {
        if self.mode == RelayerMode::AlwaysRelay {
            return;
        }

        let timeout_duration = std::env::var("GRAVITY_BATCH_SENDING_SECS")
            .map(|value| Duration::from_secs(value.parse().unwrap()))
            .unwrap_or_else(|_| Duration::from_secs(3600));

        self.next_batch_send_time
            .insert(contract_address, Instant::now() + timeout_duration);
    }

    async fn get_token_price(&mut self, contract_address: &EthAddress) -> Result<U256, ()> {
        return if let Some(token_price_str) = self
            .token_price_map
            .get(&format_eth_address(*contract_address))
        {
            let token_price = U256::from_dec_str(token_price_str).map_err(|_| {
                log::error!("Unable to parse token price");
            })?;
            return Ok(token_price);
        } else {
            error!("Cannot find token price in map");
            Err(())
        };
    }
}
