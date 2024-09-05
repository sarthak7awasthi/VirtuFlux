pub struct PerformanceMetrics {
	pub vm_id: String,
	pub cpu_usage: f32,       // Percentage 
	pub memory_usage: u32,    // Memory usage in MB
	pub network_usage: u64,   // Network throughput in bytes
	pub disk_io: u64,         // Disk input/output rate in bytes per second
	pub uptime: u64,          // VM uptime in seconds
}

impl PerformanceMetrics {
	pub fn new(vm_id: String, cpu_usage: f32, memory_usage: u32, network_usage: u64, disk_io: u64, uptime: u64) -> Self {
			Self {
					vm_id,
					cpu_usage,
					memory_usage,
					network_usage,
					disk_io,
					uptime,
			}
	}
}
