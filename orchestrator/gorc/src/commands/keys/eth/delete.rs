use crate::application::APP;
use abscissa_core::{Application, Command, Options, Runnable};

#[derive(Command, Debug, Default, Options)]
pub struct DeleteEthKeyCmd {
    #[options(free, help = "delete [name]")]
    pub args: Vec<String>,
}

// Entry point for `gorc keys eth delete [name]`
// - [name] required; key name
impl Runnable for DeleteEthKeyCmd {
    fn run(&self) {
        let config = APP.config();
        let keystore = &config.keystore;

        let name = self.args.get(0).expect("name is required");
        let name = name.parse().expect("Could not parse name");
        keystore.delete(&name).expect("Could not delete key");
    }
}
