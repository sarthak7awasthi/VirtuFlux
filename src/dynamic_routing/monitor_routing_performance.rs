use crate::dynamic_routing::RoutingMetrics;
use std::sync::{Arc, Mutex};

pub struct RoutingPerformanceMonitor {
    pub metrics: Arc<Mutex<RoutingMetrics>>, 
}

impl RoutingPerformanceMonitor {
    pub fn new() -> Self {
        RoutingPerformanceMonitor {
            metrics: Arc::new(Mutex::new(RoutingMetrics::new())),
        }
    }

    pub fn monitor_routing_performance(&self) -> RoutingMetrics {
 
        let metrics = self.metrics.lock().unwrap();
        metrics.clone()
    }

    pub fn update_metrics(&self, data_size: usize, start_time: std::time::Instant, end_time: std::time::Instant) {
        let mut metrics = self.metrics.lock().unwrap();

   
        let latency = end_time.duration_since(start_time).as_millis() as u64;
        metrics.update_latency(latency);

    
        metrics.update_throughput(data_size, latency);

        // Increment packet count
        metrics.increment_packets();
    }
}
