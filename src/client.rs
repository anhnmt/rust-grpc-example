use hello_world::{greeter_client::GreeterClient, HelloRequest};
use hyper_util::rt::TokioExecutor;
use tonic_web::GrpcWebClientLayer;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Must use hyper directly...
    let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build_http();

    let svc = tower::ServiceBuilder::new()
        .layer(GrpcWebClientLayer::new())
        .service(client);

    let mut client = GreeterClient::with_origin(svc, "http://localhost:50051".try_into()?);

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    tracing::info!("Received response: {:?}", response);

    Ok(())
}