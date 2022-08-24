use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use cosmos_gravity::crypto::EthPubkey;
use ethers::utils::hex::ToHex;

/// Show an Eth Key
#[derive(Command, Debug, Default, Parser)]
pub struct ShowEthKeyCmd {
    pub args: Vec<String>,

    #[clap(short = 'n', long)]
    pub show_name: bool,
}

// Entry point for `gorc keys eth show [name]`
impl Runnable for ShowEthKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let name = self.args.get(0).expect("name is required");

        let key = config.load_ethers_wallet(name.clone());

        let pub_key = key.to_public_key();

        if self.show_name {
            print!("{}\t", name);
        }
        let pub_key = pub_key.to_bytes();
        let hex_pubkey: String = pub_key.encode_hex();
        println!("{}", hex_pubkey);
    }
}
