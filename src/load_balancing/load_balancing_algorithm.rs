#[derive(Debug, Clone)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    LeastConnections,
}

#[derive(Debug, Clone)]
pub struct RoutingConfig {
    pub algorithm: LoadBalancingAlgorithm,
    pub last_index: usize, 
}

impl RoutingConfig {
 
    pub fn new(algorithm: LoadBalancingAlgorithm) -> Self {
        RoutingConfig {
            algorithm,
            last_index: 0,
        }
    }


    pub fn set_algorithm(&mut self, algorithm: LoadBalancingAlgorithm) {
        self.algorithm = algorithm;
    }
}
