use super::show::ShowEthKeyCmd;
use crate::application::APP;
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use bip32::PrivateKey;
use k256::{pkcs8::EncodePrivateKey, SecretKey};

///Import an Eth Key
#[derive(Command, Debug, Default, Parser)]
pub struct ImportEthKeyCmd {
    pub args: Vec<String>,

    #[clap(short, long)]
    pub overwrite: bool,
}

// Entry point for `gorc keys eth import [name] (private-key)`
// - [name] required; key name
// - (private-key) optional; when absent the user will be prompted to enter it
impl Runnable for ImportEthKeyCmd {
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

        let key = match self.args.get(1) {
            Some(private_key) => private_key.clone(),
            None => rpassword::read_password_from_tty(Some("> Enter your private-key:\n"))
                .expect("Could not read private-key"),
        };

        let key = key
            .parse::<clarity::PrivateKey>()
            .expect("Could not parse private-key");

        let key = SecretKey::from_bytes(&key.to_bytes()).expect("Could not convert private-key");

        let key = key
            .to_pkcs8_der()
            .expect("Could not PKCS8 encod private key");

        keystore.store(&name, &key).expect("Could not store key");

        let show_cmd = ShowEthKeyCmd {
            args: vec![name.to_string()],
            show_name: false,
        };
        show_cmd.run();
    }
}
