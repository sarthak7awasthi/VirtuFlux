use crate::monitoring_analytics::{trigger_alert, collect_metrics, AlertType};
use crate::fault_tolerance::{initiate_failover, detect_node_failure, recover_data};
use crate::virtualization_management::VMHandle;
use crate::data_partitioning_replication::replication::ReplicatedChunk;
use std::collections::HashMap;

pub fn handle_alerts_and_failovers(
    vm_handle: &VMHandle,
    task_id: &str,
) -> Result<(), String> {
    
    let metrics = collect_metrics(vm_handle).map_err(|e| format!("Failed to collect metrics: {:?}", e))?;


    let alert = trigger_alert(&metrics);
    
    if let Some(alert) = alert {
        match alert.alert_type {
            AlertType::NodeFailure => {
                println!("Node failure detected for VM: {:?}", vm_handle);
                
            
                let node_failure = detect_node_failure(vm_handle).map_err(|e| format!("Failed to detect node failure: {:?}", e))?;
                
                if node_failure {
                    println!("Initiating failover for VM: {:?}", vm_handle);

             
                    initiate_failover(vm_handle).map_err(|e| format!("Failed to initiate failover: {:?}", e))?;
                    
                 
                    let recovered_data = recover_data(task_id).map_err(|e| format!("Failed to recover data: {:?}", e))?;
                    println!("Recovered Data for task {}: {:?}", task_id, recovered_data);
                }
            },
            AlertType::HighCpuUsage => {
                println!("High CPU usage alert for VM: {:?}", vm_handle);
         
            },
            AlertType::HighMemoryUsage => {
                println!("High memory usage alert for VM: {:?}", vm_handle);
             
            },
            _ => {
                println!("Unhandled alert type for VM: {:?}", vm_handle);
            }
        }
    } else {
        println!("No alert triggered for VM: {:?}", vm_handle);
    }

    Ok(())
}
