mod virtualization_management;
mod dynamic_routing;
mod load_balancing;
mod fault_tolerance;
mod data_partitioning_replication;
mod monitoring_analytics;
mod control_plane; 

use control_plane::expose_api::expose_api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting VirtuFlux API...");
    expose_api().await
}
