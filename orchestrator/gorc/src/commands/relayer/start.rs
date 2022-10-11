use crate::{application::APP, prelude::*};
use abscissa_core::{clap::Parser, Command, Runnable};
use ethers::{prelude::*, types::Address as EthAddress};
use gravity_utils::types::config::RelayerMode;
use gravity_utils::{
    connection_prep::{check_for_eth, create_rpc_connections, wait_for_cosmos_node_ready},
    ethereum::{downcast_to_u64, format_eth_address},
};
use relayer::fee_manager::FeeManager;
use relayer::main_loop::{relayer_main_loop, LOOP_SPEED as RELAYER_LOOP_SPEED};
use std::str::FromStr;
use std::sync::Arc;

/// Start the relayer
#[derive(Command, Debug, Parser)]
pub struct StartCommand {
    #[clap(short, long)]
    ethereum_key: String,

    #[clap(short, long)]
    mode: Option<String>,
}

impl Runnable for StartCommand {
    fn run(&self) {
        openssl_probe::init_ssl_cert_env_vars();

        let mode_str = self.mode.as_deref().unwrap_or("Api");
        let mode = RelayerMode::from_str(mode_str)
            .expect("Incorrect mode, possible value are: AlwaysRelay, Api or File");
        info!("Relayer using mode {:?}", mode);

        let config = APP.config();
        let cosmos_prefix = config.cosmos.prefix.clone();

        let ethereum_wallet = config.load_ethers_wallet(self.ethereum_key.clone());
        let ethereum_address = ethereum_wallet.address();

        let contract_address: EthAddress = config
            .gravity
            .contract
            .parse()
            .expect("Could not parse gravity contract address");

        let mut payment_address: EthAddress = config
            .gravity
            .payment_address
            .parse()
            .expect("Could not parse gravity contract address");

        let timeout = RELAYER_LOOP_SPEED;

        abscissa_tokio::run_with_actix(&APP, async {
            let connections = create_rpc_connections(
                cosmos_prefix,
                Some(config.cosmos.grpc.clone()),
                Some(config.ethereum.rpc.clone()),
                timeout,
            )
            .await;

            let grpc = connections.grpc.clone().unwrap();
            let contact = connections.contact.clone().unwrap();
            let provider = connections.eth_provider.clone().unwrap();
            let chain_id = provider
                .get_chainid()
                .await
                .expect("Could not retrieve chain ID during relayer start");
            let chain_id =
                downcast_to_u64(chain_id).expect("Chain ID overflowed when downcasting to u64");
            let eth_client =
                SignerMiddleware::new(provider, ethereum_wallet.clone().with_chain_id(chain_id));
            let eth_client = Arc::new(eth_client);

            // if payment address is zero, then use the ethereum key address used for signing tx
            if payment_address == EthAddress::zero() {
                info!("relayer payment address is zero, use signing ethereum address instead");
                payment_address = eth_client.address()
            }

            info!("Starting Relayer");
            info!("Ethereum Address: {}", format_eth_address(ethereum_address));

            // check if the cosmos node is syncing, if so wait for it
            // we can't move any steps above this because they may fail on an incorrect
            // historic chain state while syncing occurs
            wait_for_cosmos_node_ready(&contact).await;
            check_for_eth(ethereum_address, eth_client.clone()).await;

            let mut fee_manager = FeeManager::new_fee_manager(mode).await.unwrap();
            relayer_main_loop(
                eth_client,
                grpc,
                contract_address,
                payment_address,
                config.ethereum.gas_price_multiplier,
                &mut fee_manager,
                config.ethereum.gas_multiplier,
                config.ethereum.blocks_to_search,
            )
            .await;
        })
        .unwrap_or_else(|e| {
            status_err!("executor exited with error: {}", e);
            std::process::exit(1);
        });
    }
}
