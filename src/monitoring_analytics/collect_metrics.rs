use crate::monitoring_analytics::performance_metrics::PerformanceMetrics;
use crate::virtualization_management::vm_handle::VMHandle;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng; 

pub fn collect_metrics(vm_handle: &VMHandle) -> Result<PerformanceMetrics, String> {
   
    let mut rng = rand::thread_rng();

   
    let cpu_usage = rng.gen_range(0.0..100.0);    
    let memory_usage = rng.gen_range(500..4096); 
    let network_usage = rng.gen_range(1000..10000); 
    let disk_io = rng.gen_range(500..5000);         


    let uptime = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_secs();

 
    let metrics = PerformanceMetrics::new(
        vm_handle.vm_id.clone(),
        cpu_usage,
        memory_usage,
        network_usage,
        disk_io,
        uptime,
    );

    Ok(metrics)
}
