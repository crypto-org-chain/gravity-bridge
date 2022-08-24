use super::show::ShowCosmosKeyCmd;
use crate::application::APP;
use crate::config::Keystore;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use k256::pkcs8::EncodePrivateKey;
use rand_core::OsRng;

/// Add a new Cosmos Key
#[derive(Command, Debug, Default, Parser)]
pub struct AddCosmosKeyCmd {
    pub args: Vec<String>,

    #[clap(short, long)]
    pub overwrite: bool,
}

impl Runnable for AddCosmosKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = &config.keystore;

        let name = self.args.get(0).expect("name is required");
        let name = name.parse().expect("Could not parse name");
        if let Ok(_info) = keystore.info(&name) {
            if !self.overwrite {
                eprintln!("Key already exists, exiting.");
                return;
            }
        }

        let mnemonic = bip32::Mnemonic::random(&mut OsRng, Default::default());
        match &config.keystore {
            Keystore::File(_path) => {
                eprintln!("**Important** record this bip39-mnemonic in a safe place:");
                println!("{}", mnemonic.phrase());
            }
            Keystore::Aws => {}
        }

        let seed = mnemonic.to_seed("");

        let path = config.cosmos.key_derivation_path.clone();
        let path = path
            .parse::<bip32::DerivationPath>()
            .expect("Could not parse derivation path");

        let key = bip32::XPrv::derive_from_path(seed, &path).expect("Could not derive key");
        let key = k256::SecretKey::from(key.private_key());
        let key = key
            .to_pkcs8_der()
            .expect("Could not PKCS8 encod private key");

        keystore.store(&name, &key).expect("Could not store key");

        let args = vec![name.to_string()];
        let show_cmd = ShowCosmosKeyCmd { args };
        show_cmd.run();
    }
}
