use tonic::{Request, Response, Status};
use tonic::transport::Server;

use proto::{CalculatorRequest, CalculatorResponse};
use proto::admin_server::{Admin, AdminServer};
use proto::calculator_server::{Calculator, CalculatorServer};
use proto::GetCountRequest;
use proto::GetCountResponse;

mod client;

mod proto {
    tonic::include_proto!("calculator");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

type State = std::sync::Arc<tokio::sync::RwLock<u64>>;

#[derive(Debug, Default)]
struct CalculatorService {
    state: State,
}

impl CalculatorService {
    async fn increment(&self) {
        let mut count = self.state.write().await;
        *count += 1;
        println!("Counter: {}", *count);
    }
}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    async fn add(
        &self,
        request: Request<CalculatorRequest>,
    ) -> Result<Response<CalculatorResponse>, Status> {
        self.increment().await;

        let input = request.get_ref();

        let response = CalculatorResponse {
            result: input.a + input.b,
        };

        Ok(Response::new(response))
    }

    async fn divide(
        &self,
        request: Request<CalculatorRequest>,
    ) -> Result<Response<CalculatorResponse>, Status> {
        self.increment().await;

        let input = request.get_ref();

        if input.b == 0 {
            return Err(Status::invalid_argument("cannot divide by zero"));
        }

        let resp = CalculatorResponse {
            result: input.a / input.b,
        };

        let response = Response::new(resp);

        Ok(response)
    }
}

#[derive(Debug, Default)]
struct AdminService {
    state: State,
}

#[tonic::async_trait]
impl Admin for AdminService {
    async fn get_count(
        &self,
        _request: Request<GetCountRequest>,
    ) -> Result<Response<GetCountResponse>, Status> {
        let res = self.state.read().await;
        let response = Response::new(GetCountResponse { count: *res });
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;

    let state = State::default();
    let calc = CalculatorService {
        state: state.clone(),
    };

    let admin = AdminService {
        state: state.clone(),
    };

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;

    Server::builder()
        .add_service(CalculatorServer::new(calc))
        .add_service(AdminServer::new(admin))
        .add_service(service)
        .serve(addr)
        .await?;

    Ok(())
}
