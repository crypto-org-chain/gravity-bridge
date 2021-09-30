use clarity::address::Address as EthAddress;
use clarity::PrivateKey as EthPrivateKey;
use clarity::Uint256;

use cosmos_gravity::query::get_latest_transaction_batches;
use cosmos_gravity::query::get_transaction_batch_signatures;
use ethereum_gravity::utils::GasCost;
use ethereum_gravity::utils::{downcast_to_u128, get_tx_batch_nonce};
use ethereum_gravity::{one_eth, submit_batch::send_eth_transaction_batch};
use gravity_proto::gravity::query_client::QueryClient as GravityQueryClient;
use gravity_utils::message_signatures::encode_tx_batch_confirm_hashed;
use gravity_utils::types::Erc20Token;
use gravity_utils::types::Valset;
use gravity_utils::types::{BatchConfirmResponse, TransactionBatch};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use std::time::Instant;
use tonic::transport::Channel;
use web30::client::Web3;
use web30::types::SendTxOption;

#[derive(Debug, Clone)]
struct SubmittableBatch {
    batch: TransactionBatch,
    sigs: Vec<BatchConfirmResponse>,
}

/// This function relays batches from Cosmos to Ethereum. First we request
/// the latest transaction batches, which is a list of the latest 100 batches
/// of all types. From there we determine which batches are valid to submit as
/// far as signatures and then make requests to Ethereum to determine which are
/// valid to submit given the current chain state. From there we simulate a submission
/// and if that succeeds and we like the gas cost we complete the relaying process and
/// actually submit the data to Ethereum
pub async fn relay_batches(
    // the validator set currently in the contract on Ethereum
    current_valset: Valset,
    ethereum_key: EthPrivateKey,
    web3: &Web3,
    grpc_client: &mut GravityQueryClient<Channel>,
    gravity_contract_address: EthAddress,
    gravity_id: String,
    timeout: Duration,
    gas_multiplier: f32,
    next_batch_send_time: &mut HashMap<EthAddress, Instant>,
) {
    let possible_batches =
        get_batches_and_signatures(current_valset.clone(), grpc_client, gravity_id.clone()).await;

    trace!("possible batches {:?}", possible_batches);

    submit_batches(
        current_valset,
        ethereum_key,
        web3,
        gravity_contract_address,
        gravity_id,
        timeout,
        gas_multiplier,
        possible_batches,
        next_batch_send_time,
    )
    .await;
}

/// This function retrieves the latest batches from the Cosmos module and then
/// iterates through the signatures for each batch, determining if they are ready
/// to submit. It is possible for a batch to not have valid signatures for two reasons
/// one is that not enough signatures have been collected yet from the validators two is
/// that the batch is old enough that the signatures do not reflect the current validator
/// set on Ethereum. In both the later and the former case the correct solution is to wait
/// through timeouts, new signatures, or a later valid batch being submitted old batches will
/// always be resolved.
async fn get_batches_and_signatures(
    current_valset: Valset,
    grpc_client: &mut GravityQueryClient<Channel>,
    gravity_id: String,
) -> HashMap<EthAddress, Vec<SubmittableBatch>> {
    let latest_batches = if let Ok(lb) = get_latest_transaction_batches(grpc_client).await {
        lb
    } else {
        return HashMap::new();
    };
    trace!("Latest batches {:?}", latest_batches);

    let mut possible_batches = HashMap::new();
    for batch in latest_batches {
        let sigs =
            get_transaction_batch_signatures(grpc_client, batch.nonce, batch.token_contract).await;
        trace!("Got sigs {:?}", sigs);
        if let Ok(sigs) = sigs {
            // this checks that the signatures for the batch are actually possible to submit to the chain
            let hash = encode_tx_batch_confirm_hashed(gravity_id.clone(), batch.clone());
            if current_valset.order_sigs(&hash, &sigs).is_ok() {
                // we've found a valid batch, add it to the list for it's token type
                possible_batches
                    .entry(batch.token_contract)
                    .or_insert_with(Vec::new);

                let list = possible_batches.get_mut(&batch.token_contract).unwrap();
                list.push(SubmittableBatch { batch, sigs });
            } else {
                warn!(
                    "Batch {}/{} can not be submitted yet, waiting for more signatures",
                    batch.token_contract, batch.nonce
                );
            }
        } else {
            error!(
                "could not get signatures for {}:{} with {:?}",
                batch.token_contract, batch.nonce, sigs
            );
        }
    }
    // reverse the list so that it is oldest first, we want to submit
    // older batches so that we don't invalidate newer batches
    for (_key, value) in possible_batches.iter_mut() {
        value.reverse();
    }
    return possible_batches;
}

/// Attempts to submit batches with valid signatures, checking the state
/// of the Ethereum chain to ensure that it is valid to submit a given batch
/// more specifically that the correctly signed batch has not timed out or already
/// been submitted. The goal of this function is to submit batches in chronological order
/// of their creation, submitting batches newest first will invalidate old batches and is
/// less efficient if those old batches are profitable.
/// This function estimates the cost of submitting a batch before actually submitting it
/// to Ethereum, if it is determined that the ETH cost to submit is too high the batch will
/// be skipped and a later, more profitable, batch may be submitted.
/// Keep in mind that many other relayers are making this same computation and some may have
/// different standards for their profit margin, therefore there may be a race not only to
/// submit individual batches but also batches in different orders
async fn submit_batches(
    current_valset: Valset,
    ethereum_key: EthPrivateKey,
    web3: &Web3,
    gravity_contract_address: EthAddress,
    gravity_id: String,
    timeout: Duration,
    gas_multiplier: f32,
    possible_batches: HashMap<EthAddress, Vec<SubmittableBatch>>,
    next_batch_send_time: &mut HashMap<EthAddress, Instant>,
) {
    let our_ethereum_address = ethereum_key.to_public_key().unwrap();
    let ethereum_block_height = if let Ok(bn) = web3.eth_block_number().await {
        bn
    } else {
        warn!("Failed to get eth block height, is your eth node working?");
        return;
    };

    // requests data from Ethereum only once per token type, this is valid because we are
    // iterating from oldest to newest, so submitting a batch earlier in the loop won't
    // ever invalidate submitting a batch later in the loop. Another relayer could always
    // do that though.
    for (token_type, possible_batches) in possible_batches {
        let erc20_contract = token_type;
        let latest_ethereum_batch = get_tx_batch_nonce(
            gravity_contract_address,
            erc20_contract,
            our_ethereum_address,
            web3,
        )
        .await;
        if latest_ethereum_batch.is_err() {
            error!(
                "Failed to get latest Ethereum batch with {:?}",
                latest_ethereum_batch
            );
            return;
        }
        let latest_ethereum_batch = latest_ethereum_batch.unwrap();

        for batch in possible_batches {
            let oldest_signed_batch = batch.batch;
            let oldest_signatures = batch.sigs;

            let timeout_height: Uint256 = oldest_signed_batch.batch_timeout.into();
            if timeout_height < ethereum_block_height {
                warn!(
                    "Batch {}/{} has timed out and can not be submitted",
                    oldest_signed_batch.nonce, oldest_signed_batch.token_contract
                );
                continue;
            }

            let latest_cosmos_batch_nonce = oldest_signed_batch.clone().nonce;
            if latest_cosmos_batch_nonce > latest_ethereum_batch {
                let cost = ethereum_gravity::submit_batch::estimate_tx_batch_cost(
                    current_valset.clone(),
                    oldest_signed_batch.clone(),
                    &oldest_signatures,
                    web3,
                    gravity_contract_address,
                    gravity_id.clone(),
                    ethereum_key,
                )
                .await;
                if cost.is_err() {
                    error!("Batch cost estimate failed with {:?}", cost);
                    continue;
                }
                let cost = cost.unwrap();

                if can_send_batch(
                    &cost,
                    &oldest_signed_batch.total_fee,
                    &oldest_signed_batch.token_contract,
                    next_batch_send_time,
                )
                .await
                {
                    let token_contract = oldest_signed_batch.token_contract;

                    info!(
                        "We have detected latest batch {} but latest on Ethereum is {} This batch is estimated to cost {} Gas / {:.4} ETH to submit",
                        latest_cosmos_batch_nonce,
                        latest_ethereum_batch,
                        cost.gas_price.clone(),
                        downcast_to_u128(cost.get_total()).unwrap() as f32
                            / downcast_to_u128(one_eth()).unwrap() as f32
                    );

                    let tx_options = vec![SendTxOption::GasPriceMultiplier(gas_multiplier)];

                    let res = send_eth_transaction_batch(
                        current_valset.clone(),
                        oldest_signed_batch,
                        &oldest_signatures,
                        web3,
                        timeout,
                        gravity_contract_address,
                        gravity_id.clone(),
                        ethereum_key,
                        tx_options,
                    )
                    .await;

                    if res.is_err() {
                        info!("Batch submission failed with {:?}", res);
                    } else {
                        update_next_batch_send_time(next_batch_send_time, token_contract)
                    }
                }
            }
        }
    }
}

async fn can_send_batch(
    estimated_cost: &GasCost,
    batch_fee: &Erc20Token,
    contract_address: &EthAddress,
    next_batch_send_time: &mut HashMap<EthAddress, Instant>,
) -> bool {
    match next_batch_send_time.get(contract_address) {
        Some(time) => {
            if *time < Instant::now() {
                return true;
            }
        }
        None => update_next_batch_send_time(next_batch_send_time, *contract_address),
    }

    let token_price = match get_token_price(&batch_fee.token_contract_address).await {
        Ok(token_price) => token_price,
        Err(_) => return false,
    };

    let estimated_fee = estimated_cost.get_total();
    let batch_value = batch_fee.amount.clone() * token_price;

    batch_value >= estimated_fee
}

fn update_next_batch_send_time(
    next_batch_send_time: &mut HashMap<EthAddress, Instant>,
    contract_address: EthAddress,
) {
    let timeout_duration = std::env::var("GRAVITY_BATCH_SENDING_SECS")
        .map(|value| Duration::from_secs(value.parse().unwrap()))
        .unwrap_or_else(|_| Duration::from_secs(3600));

    next_batch_send_time.insert(contract_address, Instant::now() + timeout_duration);
}

async fn get_token_price(contract_address: &EthAddress) -> Result<Uint256, ()> {
    // TODO: Use API for fetching price instead of json config file

    let config_file_path =
        std::env::var("GRAVITY_TOKEN_PRICES").unwrap_or_else(|_| "token_prices.json".to_owned());

    let config_str = match tokio::fs::read_to_string(config_file_path).await {
        Err(err) => {
            log::error!("Error while fetching token price: {}", err);
            return Err(());
        }
        Ok(value) => value,
    };

    let config: serde_json::Map<String, serde_json::Value> = match serde_json::from_str(&config_str)
    {
        Err(err) => {
            log::error!(
                "Error while parsing token prices json configuration: {}",
                err
            );
            return Err(());
        }
        Ok(config) => config,
    };

    let token_price = config
        .get(&contract_address.to_string())
        .ok_or_else(|| ())?;

    if !token_price.is_string() {
        log::error!("Expected token price in string format");
        return Err(());
    }

    match token_price.as_str() {
        None => {
            log::error!("Expected token price in string format");
            Err(())
        }
        Some(token_price_str) => {
            let token_price = Uint256::from_str(token_price_str);

            if token_price.is_err() {
                log::error!("Unable to parse token price");
            }

            token_price.map_err(|_| ())
        }
    }
}
