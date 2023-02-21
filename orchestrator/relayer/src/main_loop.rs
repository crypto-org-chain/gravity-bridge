use crate::fee_manager::FeeManager;
use crate::{
    batch_relaying::relay_batches, find_latest_valset::find_latest_valset,
    logic_call_relaying::relay_logic_calls, valset_relaying::relay_valsets,
};
use ethereum_gravity::{logic_call::LogicCallSkips, types::EthClient, utils::get_gravity_id};
use ethers::signers::Signer;
use ethers::types::Address as EthAddress;
use gravity_proto::gravity::query_client::QueryClient as GravityQueryClient;
use std::time::Duration;
use tonic::transport::Channel;

pub const LOOP_SPEED: Duration = Duration::from_secs(17);
pub const PENDING_TX_TIMEOUT: Duration = Duration::from_secs(120);

/// This function contains the orchestrator primary loop, it is broken out of the main loop so that
/// it can be called in the test runner for easier orchestration of multi-node tests
#[allow(unused_variables)]
#[allow(clippy::too_many_arguments)]
pub async fn relayer_main_loop<S: Signer + 'static>(
    eth_client: EthClient<S>,
    grpc_client: GravityQueryClient<Channel>,
    gravity_contract_address: EthAddress,
    payment_address: EthAddress,
    eth_gas_price_multiplier: f32,
    fee_manager: &mut FeeManager,
    eth_gas_multiplier: f32,
    blocks_to_search: u64,
    supported_contracts: Vec<EthAddress>,
) {
    let mut grpc_client = grpc_client;

    let gravity_id = get_gravity_id(
        gravity_contract_address,
        eth_client.clone(),
        grpc_client.clone(),
    )
    .await;
    if let Err(e) = gravity_id {
        error!("Error when fetching the GravityID {e}");
        return;
    }
    let gravity_id = gravity_id.unwrap();
    let mut logic_call_skips = LogicCallSkips::default();

    loop {
        let (async_resp, _) = tokio::join!(
            async {
                let current_eth_valset = find_latest_valset(
                    &mut grpc_client,
                    gravity_contract_address,
                    eth_client.clone(),
                    blocks_to_search,
                )
                .await;
                if current_eth_valset.is_err() {
                    error!("Could not get current valset! {:?}", current_eth_valset);
                    return;
                }
                let current_eth_valset = current_eth_valset.unwrap();

                relay_valsets(
                    current_eth_valset.clone(),
                    eth_client.clone(),
                    &mut grpc_client,
                    gravity_contract_address,
                    gravity_id.clone(),
                    PENDING_TX_TIMEOUT,
                    eth_gas_price_multiplier,
                    eth_gas_multiplier,
                )
                .await;

                relay_batches(
                    current_eth_valset.clone(),
                    eth_client.clone(),
                    &mut grpc_client,
                    gravity_contract_address,
                    payment_address,
                    gravity_id.clone(),
                    PENDING_TX_TIMEOUT,
                    eth_gas_price_multiplier,
                    fee_manager,
                    eth_gas_multiplier,
                    supported_contracts.clone(),
                )
                .await;

                relay_logic_calls(
                    current_eth_valset,
                    eth_client.clone(),
                    &mut grpc_client,
                    gravity_contract_address,
                    payment_address,
                    gravity_id.clone(),
                    PENDING_TX_TIMEOUT,
                    eth_gas_price_multiplier,
                    eth_gas_multiplier,
                    &mut logic_call_skips,
                )
                .await;
            },
            tokio::time::sleep(LOOP_SPEED)
        );
    }
}
