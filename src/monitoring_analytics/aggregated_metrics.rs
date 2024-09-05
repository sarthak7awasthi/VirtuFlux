use crate::monitoring_analytics::performance_metrics::PerformanceMetrics;

pub struct AggregatedMetrics {
    pub average_cpu_usage: f32,
    pub total_memory_usage: u32,
    pub total_network_usage: u64,
    pub total_disk_io: u64,
    pub total_uptime: u64,
    pub vm_count: usize,
}

impl AggregatedMetrics {
    pub fn new() -> Self {
        Self {
            average_cpu_usage: 0.0,
            total_memory_usage: 0,
            total_network_usage: 0,
            total_disk_io: 0,
            total_uptime: 0,
            vm_count: 0,
        }
    }

    pub fn add_metrics(&mut self, metrics: &PerformanceMetrics) {
        self.total_memory_usage += metrics.memory_usage;
        self.total_network_usage += metrics.network_usage;
        self.total_disk_io += metrics.disk_io;
        self.total_uptime += metrics.uptime;
        self.average_cpu_usage = 
            ((self.average_cpu_usage * self.vm_count as f32) + metrics.cpu_usage)
            / (self.vm_count as f32 + 1.0);
        self.vm_count += 1;
    }
}
