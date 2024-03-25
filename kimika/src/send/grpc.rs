use crate::utils::{
    color::{print_color, Color},
    utils_type::TonicRes,
};
use kimika_grpc::local::{local_client::LocalClient, EmptyResponse, MessageRequest};
use tonic::transport::Channel;

#[allow(unused_variables)]
pub async fn send_file(client: &mut LocalClient<Channel>, path: String) -> TonicRes<EmptyResponse> {
    todo!()
}

pub async fn send_message(client: &mut LocalClient<Channel>, message: String) {
    client
        .send_message(MessageRequest { message })
        .await
        .expect("Message sending failed");
    print_color("Message sent successfully", Color::Green)
}
