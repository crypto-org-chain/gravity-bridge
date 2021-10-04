use super::show::ShowEthKeyCmd;
use crate::application::APP;
use abscissa_core::{Application, Command, Options, Runnable};
use k256::pkcs8::ToPrivateKey;
use rand_core::OsRng;

#[derive(Command, Debug, Default, Options)]
pub struct AddEthKeyCmd {
    #[options(free, help = "add [name]")]
    pub args: Vec<String>,

    #[options(help = "overwrite existing key")]
    pub overwrite: bool,

    #[options(help = "show private key")]
    show_private_key: bool,
}

// Entry point for `gorc keys eth add [name]`
// - [name] required; key name
impl Runnable for AddEthKeyCmd {
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
        eprintln!("**Important** record this bip39-mnemonic in a safe place:");
        println!("{}", mnemonic.phrase());

        let seed = mnemonic.to_seed("");

        let path = config.ethereum.key_derivation_path.trim();
        let path = path
            .parse::<bip32::DerivationPath>()
            .expect("Could not parse derivation path");

        let key = bip32::XPrv::derive_from_path(seed, &path).expect("Could not derive key");
        let key = k256::SecretKey::from(key.private_key());
        let key = key
            .to_pkcs8_der()
            .expect("Could not PKCS8 encod private key");

        keystore.store(&name, &key).expect("Could not store key");

        let show_cmd = ShowEthKeyCmd {
            args: vec![name.to_string()],
            show_private_key: self.show_private_key,
            show_name: false,
        };
        show_cmd.run();
    }
}
