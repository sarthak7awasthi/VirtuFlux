use crate::monitoring_analytics::aggregated_metrics::AggregatedMetrics;
use crate::monitoring_analytics::alert::Alert;

pub struct PerformanceReport {
    pub time_range: String,  
    pub aggregated_metrics: AggregatedMetrics,
    pub triggered_alerts: Vec<Alert>,  
}

impl PerformanceReport {
    pub fn new(time_range: String, aggregated_metrics: AggregatedMetrics, triggered_alerts: Vec<Alert>) -> Self {
        Self {
            time_range,
            aggregated_metrics,
            triggered_alerts,
        }
    }

    pub fn display_report(&self) {
        println!("Performance Report for: {}", self.time_range);
        println!("Aggregated Metrics:");
        println!("Average CPU Usage: {:.2}%", self.aggregated_metrics.average_cpu_usage);
        println!("Total Memory Usage: {} MB", self.aggregated_metrics.total_memory_usage);
        println!("Total Network Usage: {} bytes", self.aggregated_metrics.total_network_usage);
        println!("Total Disk I/O: {} bytes/s", self.aggregated_metrics.total_disk_io);
        println!("Total Uptime: {} seconds", self.aggregated_metrics.total_uptime);
        println!("Number of VMs: {}", self.aggregated_metrics.vm_count);

        if !self.triggered_alerts.is_empty() {
            println!("\nAlerts triggered during this period:");
            for alert in &self.triggered_alerts {
                println!(
                    "Alert: {:?}, Description: {}, Severity: {}, Timestamp: {}",
                    alert.alert_type, alert.description, alert.severity, alert.timestamp
                );
            }
        } else {
            println!("\nNo alerts triggered during this period.");
        }
    }
}
