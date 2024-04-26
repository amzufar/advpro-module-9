use tonic::{transport::Server, Request, Response, Status};
use services::{payment_service_server::{PaymentService, PaymentServiceServer}, PaymentRequest, PaymentResponse};

pub mod services {
    tonic::include_proto!("service");
}

#[derive(Default)]
pub struct MyPamentService {}

#[tonic::async_trait]
impl PaymentService for MyPamentService {
    async fn process_payment(
        &self,
        request: Request<PaymentRequest>,
    ) -> Result<Response<PaymentResponse>, Status> {
        println!("Received payment request: {:?}", request);

        // Process the request and return a response
        // This example immediately returns a successful result for demonstration purposes
        Ok(Response::new(PaymentResponse {success: true }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let payment_service = MyPamentService::default();

    Server::builder()
        .add_service(PaymentServiceServer::new(payment_service))
        .serve(addr)
        .await?;

    Ok(())
}