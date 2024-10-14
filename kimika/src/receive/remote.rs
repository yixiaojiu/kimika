use super::ReceiveArgs;
use crate::request::remote as request_remote;
use crate::{config, utils::handle};
use crossterm::style::Stylize;

pub async fn remote_receive(
    args: &ReceiveArgs,
    config: &config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let address = if let Some(addr) = handle::handle_address(args.address.clone(), config) {
        addr
    } else {
        println!("{}", "No server address configured".red());
        return Ok(());
    };

    let request = request_remote::RequestClient::new(&address);

    let receiver_id = request.post_register(config.alias.clone()).await?.id;

    Ok(())
}
