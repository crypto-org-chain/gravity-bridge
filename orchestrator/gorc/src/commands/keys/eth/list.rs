use super::show::ShowEthKeyCmd;
use crate::{application::APP, config::Keystore};
use abscissa_core::{clap::Parser, Application, Command, Runnable};
use std::path;

/// List all Eth Keys
#[derive(Command, Debug, Default, Parser)]
pub struct ListEthKeyCmd {}

// Entry point for `gorc keys eth list`
impl Runnable for ListEthKeyCmd {
    fn run(&self) {
        let config = APP.config();
        if let Keystore::File(path) = &config.keystore {
            let keystore = path::Path::new(&path);

            for entry in keystore.read_dir().expect("Could not read keystore") {
                let path = entry.unwrap().path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "pem" {
                            let name = path.file_stem().unwrap();
                            let name = name.to_str().unwrap();
                            let show_cmd = ShowEthKeyCmd {
                                args: vec![name.to_string()],
                                show_name: true,
                            };
                            show_cmd.run();
                        }
                    }
                }
            }
        }
    }
}
