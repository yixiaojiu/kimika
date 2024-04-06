mod grpc;

// use kimika_grpc::remote::remote_server::RemoteServer;
// use tonic::transport::Server;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let address = "0.0.0.0:3940".parse()?;

    // let remote_service = grpc::RemoteService::default();
    // Server::builder()
    //     .add_service(RemoteServer::new(remote_service))
    //     .serve(address)
    //     .await?;

    Ok(())
}
