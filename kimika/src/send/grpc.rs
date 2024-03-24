use crate::transfer::{transfer_client::TransferClient, EmptyResponse, MessageRequest};
use crate::utils::{
    color::{print_color, Color},
    utils_type::TonicRes,
};
use tonic::transport::Channel;

#[allow(unused_variables)]
pub async fn send_file(
    client: &mut TransferClient<Channel>,
    path: String,
) -> TonicRes<EmptyResponse> {
    todo!()
}

pub async fn send_message(client: &mut TransferClient<Channel>, message: String) {
    client
        .send_message(MessageRequest { message })
        .await
        .expect("send message failed");
    print_color("message sent successfully", Color::Green)
}
