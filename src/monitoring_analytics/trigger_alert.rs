use crate::monitoring_analytics::performance_metrics::PerformanceMetrics;
use crate::monitoring_analytics::alert::{Alert, AlertType};

pub fn trigger_alert(metrics: &PerformanceMetrics) -> Option<Alert> {
   
    let cpu_threshold = 90.0;     
    let memory_threshold = 3500; 
    let network_threshold = 8000;  


    if metrics.cpu_usage > cpu_threshold {
        return Some(Alert::new(
            AlertType::HighCpuUsage,
            format!("High CPU usage detected: {:.2}%", metrics.cpu_usage),
            9, 
        ));
    }

   
    if metrics.memory_usage > memory_threshold {
        return Some(Alert::new(
            AlertType::HighMemoryUsage,
            format!("High memory usage detected: {}MB", metrics.memory_usage),
            7,
        ));
    }


    if metrics.network_usage > network_threshold {
        return Some(Alert::new(
            AlertType::HighNetworkUsage,
            format!("High network usage detected: {}B", metrics.network_usage),
            5, 
        ));
    }

    None
}
