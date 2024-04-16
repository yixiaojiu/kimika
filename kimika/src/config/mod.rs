use crate::{receive, send};
use figment::{
    providers::{Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub receiver: Option<ReceiverConfig>,
    pub sender: Option<SenderConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub alias: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReceiverConfig {
    pub alias: Option<String>,
    pub save_folder: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct SenderConfig {
    pub alias: Option<String>,
    pub port: Option<u16>,
    pub receiver_port: Option<u16>,
}

impl Config {
    pub fn new() -> Self {
        let user_config_path = dirs::home_dir()
            .map(|p| p.join(".config/kimika/config.toml"))
            .expect("get home dir failed.");
        if user_config_path.exists() {
            return Figment::new()
                .merge(Toml::string(include_str!("./default.toml")))
                .merge(Toml::file(user_config_path))
                .extract()
                .unwrap();
        }

        Figment::new()
            .merge(Toml::string(include_str!("./default.toml")))
            .extract()
            .unwrap()
    }

    pub fn update_from_send_args(&mut self, args: &send::SendArgs) {
        if let Some(address) = &args.address {
            self.server.as_mut().unwrap().address = Some(address.clone())
        }
        if let Some(port) = args.port {
            self.sender.as_mut().unwrap().port = Some(port)
        }
        if let Some(receiver_port) = args.receiver_port {
            self.sender.as_mut().unwrap().receiver_port = Some(receiver_port)
        }
    }

    pub fn update_from_receive_args(&mut self, args: &receive::ReceiveArgs) {
        if let Some(port) = args.port {
            self.receiver.as_mut().unwrap().port = Some(port)
        }
        if let Some(alias) = &args.alias {
            self.receiver.as_mut().unwrap().alias = Some(alias.clone())
        }
        if let Some(save_folder) = &args.save_folder {
            self.receiver.as_mut().unwrap().save_folder = Some(save_folder.clone())
        }
    }
}
