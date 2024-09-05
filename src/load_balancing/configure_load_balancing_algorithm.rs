use crate::load_balancing::load_balancing_algorithm::{LoadBalancingAlgorithm, RoutingConfig};


pub fn configure_load_balancing_algorithm(config: &mut RoutingConfig, algorithm: LoadBalancingAlgorithm) {
    config.set_algorithm(algorithm);
    println!("Switched to {:?} load balancing algorithm.", config.algorithm);
}
