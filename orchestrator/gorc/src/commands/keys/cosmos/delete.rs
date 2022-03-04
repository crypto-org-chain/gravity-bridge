use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};

/// Delete a Cosmos Key
#[derive(Command, Debug, Default, Parser)]
pub struct DeleteCosmosKeyCmd {
    pub args: Vec<String>,
}

/// The `gork keys cosmos delete [name] ` subcommand: delete the given key
impl Runnable for DeleteCosmosKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = &config.keystore;
        // Collect key name from args.
        let name = self.args.get(0).expect("name is required");
        let name = name.parse().expect("Could not parse name");
        // Delete keyname after locating file from path and key name.
        let _delete_key = keystore.delete(&name).unwrap();
    }
}
