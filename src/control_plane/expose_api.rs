use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use crate::control_plane::manage_vm_lifecycle::{manage_vm_lifecycle, VMAction};
use crate::control_plane::coordinate_routing_and_balancing::coordinate_routing_and_balancing;
use crate::control_plane::handle_alerts_and_failovers::handle_alerts_and_failovers;
use crate::virtualization_management::{VMHandle, NetworkConfig};
use crate::dynamic_routing::RoutingAlgorithm;
use std::sync::Mutex;
use lazy_static::lazy_static;
use std::env;


lazy_static! {
    static ref IP_POOL: Mutex<IpAllocator> = Mutex::new(IpAllocator::new("192.168.122.10"));
}


struct IpAllocator {
    current_ip: String,
}

impl IpAllocator {
    fn new(starting_ip: &str) -> Self {
        IpAllocator {
            current_ip: starting_ip.to_string(),
        }
    }

    fn get_next_ip(&mut self) -> String {
     
        let parts: Vec<&str> = self.current_ip.split('.').collect();
        let last_octet = parts[3].parse::<u8>().unwrap();
        let new_ip = if last_octet < 254 {
            format!("{}.{}.{}.{}", parts[0], parts[1], parts[2], last_octet + 1)
        } else {
            panic!("No more IP addresses available in the pool");
        };
        self.current_ip = new_ip.clone();
        new_ip
    }
}

#[derive(Serialize, Deserialize)]
struct CreateVMRequest {
    vm_id: String,
    cpu_cores: u8,
    memory_mb: u32,
    disk_size_gb: u32,
}

#[derive(Serialize, Deserialize)]
struct RoutingRequest {
    data_packet: Vec<u8>,
    routing_algorithm: String,
}

#[derive(Serialize, Deserialize)]
struct AlertFailoverRequest {
    vm_handle: VMHandle,
    task_id: String,
}

#[post("/create_vm")]
async fn create_vm(req: web::Json<CreateVMRequest>) -> impl Responder {

    let ip_address = {
        let mut ip_pool = IP_POOL.lock().unwrap();
        ip_pool.get_next_ip()
    };

   
    let subnet_mask = env::var("SUBNET_MASK").unwrap_or_else(|_| "255.255.255.0".to_string());

    let network_config = NetworkConfig {
        ip_address,
        subnet_mask,
    };

    let result = manage_vm_lifecycle(VMAction::Create {
        vm_id: req.vm_id.clone(),
        cpu_cores: req.cpu_cores,
        memory_mb: req.memory_mb,
        disk_size_gb: req.disk_size_gb,
        network_config,
    });

    match result {
        Ok(_) => HttpResponse::Ok().json("VM created successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create VM: {}", e)),
    }
}

#[post("/coordinate_routing")]
async fn coordinate_routing(req: web::Json<RoutingRequest>) -> impl Responder {
    let routing_algorithm = match req.routing_algorithm.as_str() {
        "LeastConnections" => RoutingAlgorithm::LeastConnections,
        "RoundRobin" => RoutingAlgorithm::RoundRobin,
        _ => return HttpResponse::BadRequest().json("Invalid routing algorithm"),
    };

    let result = coordinate_routing_and_balancing(req.data_packet.clone(), routing_algorithm);

    match result {
        Ok(_) => HttpResponse::Ok().json("Data routed successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to route data: {}", e)),
    }
}

#[post("/handle_alerts_failovers")]
async fn handle_alerts_failovers(req: web::Json<AlertFailoverRequest>) -> impl Responder {
    let result = handle_alerts_and_failovers(&req.vm_handle, &req.task_id);

    match result {
        Ok(_) => HttpResponse::Ok().json("Alert handled and failover completed"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to handle alert: {}", e)),
    }
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("API is running")
}

pub async fn expose_api() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(create_vm)
            .service(coordinate_routing)
            .service(handle_alerts_failovers)
            .service(health_check)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
