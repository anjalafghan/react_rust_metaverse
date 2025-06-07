use shared_lib::helloworld::{Greeter, GreeterServer, HelloReply, HelloRequest};
use tonic::{Request, Response, Status, transport::Server};
use tracing_subscriber;

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        tracing::info!("Got a request: {:?}", request);

        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Use a more flexible address that works for both IPv4 and IPv6
    let addr = "[::0]:50051".parse()?;
    let greeter = MyGreeter::default();

    tracing::info!("Starting gRPC server on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve_with_shutdown(addr, async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to install CTRL+C signal handler");
            tracing::info!("Shutting down server...");
        })
        .await?;

    Ok(())
}
