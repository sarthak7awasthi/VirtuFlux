#[derive(Debug, Clone)]
pub struct RoutingMetrics {
    pub latency_ms: u64,
    pub throughput: usize, 
    pub packets_routed: u64,
}

impl RoutingMetrics {
    pub fn new() -> Self {
        RoutingMetrics {
            latency_ms: 0,
            throughput: 0,
            packets_routed: 0,
        }
    }

    pub fn update_latency(&mut self, latency: u64) {
        self.latency_ms = latency;
    }

    pub fn update_throughput(&mut self, data_size: usize, time_ms: u64) {
        if time_ms > 0 {
            self.throughput = data_size * 1000 / time_ms as usize;
        }
    }

    pub fn increment_packets(&mut self) {
        self.packets_routed += 1;
    }
}
