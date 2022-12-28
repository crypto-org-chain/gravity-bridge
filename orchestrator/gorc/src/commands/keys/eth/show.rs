use crate::application::APP;
use crate::utils::aws::verifying_key_to_address;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use cosmos_gravity::crypto::EthPubkey;

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
        let address = verifying_key_to_address(&pub_key);

        if self.show_name {
            print!("{name}\t");
        }

        println!("{address:?}");
    }
}
