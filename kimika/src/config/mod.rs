use crate::{receive, send};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub alias: String,
    pub auto_select_first_server: bool,
    pub receiver: ReceiverConfig,
    pub sender: SenderConfig,
    pub server: Vec<ServerConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ServerConfig {
    pub alias: String,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ReceiverConfig {
    pub save_folder: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SenderConfig {
    pub port: u16,
    pub receiver_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let user_config_path = dirs::home_dir()
            .map(|p| p.join(".config/kimika/config.toml"))
            .expect("get home dir failed.");
        if user_config_path.exists() {
            return Figment::from(Serialized::default("server", Vec::<ServerConfig>::new()))
                .merge(Toml::string(include_str!("./default.toml")))
                .merge(Toml::file(user_config_path))
                .extract()
                .unwrap();
        }

        Figment::from(Serialized::default("server", Vec::<ServerConfig>::new()))
            .merge(Toml::string(include_str!("./default.toml")))
            .extract()
            .unwrap()
    }

    pub fn update_from_send_args(&mut self, args: &send::SendArgs) {
        if let Some(alias) = args.alias.clone() {
            self.alias = alias;
        }
        if let Some(port) = args.port {
            self.sender.port = port
        }
        if let Some(receiver_port) = args.receiver_port {
            self.sender.receiver_port = receiver_port
        }
    }

    pub fn update_from_receive_args(&mut self, args: &receive::ReceiveArgs) {
        if let Some(port) = args.port {
            self.receiver.port = port
        }
        if let Some(alias) = args.alias.clone() {
            self.alias = alias
        }
        if let Some(save_folder) = args.folder.clone() {
            self.receiver.save_folder = save_folder
        }
    }
}
