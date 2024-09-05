use std::thread::sleep;
use std::time::Duration;
use crossterm::{ExecutableCommand, terminal, ClearType, cursor};
use crate::monitoring_analytics::collect_metrics::collect_metrics;
use crate::monitoring_analytics::trigger_alert::trigger_alert;
use crate::virtualization_management::vm_handle::VMHandle;
use crate::monitoring_analytics::aggregate_metrics::aggregate_metrics;

fn display_dashboard(vm_handles: Vec<VMHandle>) {
    loop {
 
        std::io::stdout().execute(terminal::Clear(ClearType::All)).unwrap();
        std::io::stdout().execute(cursor::MoveTo(0, 0)).unwrap();

  
        let mut metrics_list = Vec::new();
        let mut triggered_alerts = Vec::new();

        for vm_handle in &vm_handles {
            if let Ok(metrics) = collect_metrics(vm_handle) {
                metrics_list.push(metrics.clone());
                
              
                if let Some(alert) = trigger_alert(&metrics) {
                    triggered_alerts.push(alert);
                }

       
                println!(
                    "VM: {}\nCPU Usage: {:.2}%\nMemory Usage: {} MB\nNetwork Usage: {} B\nDisk I/O: {} B/s\nUptime: {} s\n",
                    vm_handle.vm_id,
                    metrics.cpu_usage,
                    metrics.memory_usage,
                    metrics.network_usage,
                    metrics.disk_io,
                    metrics.uptime
                );
            }
        }

    
        let aggregated_metrics = aggregate_metrics(metrics_list);


        println!("\nAggregated Metrics:\n");
        println!("Average CPU Usage: {:.2}%", aggregated_metrics.average_cpu_usage);
        println!("Total Memory Usage: {} MB", aggregated_metrics.total_memory_usage);
        println!("Total Network Usage: {} bytes", aggregated_metrics.total_network_usage);
        println!("Total Disk I/O: {} bytes/s", aggregated_metrics.total_disk_io);
        println!("Total Uptime: {} seconds", aggregated_metrics.total_uptime);


        if !triggered_alerts.is_empty() {
            println!("\n--- Alerts ---\n");
            for alert in triggered_alerts {
                println!(
                    "Alert: {:?}\nDescription: {}\nSeverity: {}\nTimestamp: {}\n",
                    alert.alert_type, alert.description, alert.severity, alert.timestamp
                );
            }
        } else {
            println!("\nNo alerts triggered.");
        }

    
        sleep(Duration::from_secs(2));
    }
}
