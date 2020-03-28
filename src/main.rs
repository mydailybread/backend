use anyhow::Result;
use diesel::r2d2::ConnectionManager;
use lieferemma::{DriverServer, DriverServerImpl, EndCustomerServer, EndCustomerServerImpl, Opt};
use structopt::StructOpt;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();

    let pg_connection_manager = ConnectionManager::new(opt.database_url());
    let pg_connection_pool = r2d2::Pool::new(pg_connection_manager).unwrap();

    let end_customer_server = EndCustomerServerImpl { pg_connection_pool };
    let driver_server = DriverServerImpl {};

    println!("gRPC API served from {}", opt.grpc_api_addr());

    Server::builder()
        .add_service(EndCustomerServer::new(end_customer_server))
        .add_service(DriverServer::new(driver_server))
        .serve(*opt.grpc_api_addr())
        .await?;

    Ok(())
}
