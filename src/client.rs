use error::Error;
use std::error;

use proto::calculator_client::CalculatorClient;
use proto::CalculatorRequest;

pub mod proto {
    tonic::include_proto!("calculator");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "http://[::1]:50051";
    let mut client = CalculatorClient::connect(addr).await?;

    let req = CalculatorRequest { a: 1, b: 2 };

    let request = tonic::Request::new(req);
    let res = client.add(request).await?;

    println!("Result {:?}", res.get_ref().result);

    Ok(())
}
