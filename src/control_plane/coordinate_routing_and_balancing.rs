use crate::load_balancing::{calculate_load, distribute_load};
use crate::dynamic_routing::{route_data, RoutingCriteria, RoutingAlgorithm};
use crate::load_balancing::load_balancing_algorithm::LoadMetric;
use crate::monitoring_analytics::collect_metrics::collect_metrics;
use crate::virtualization_management::{VMHandle, list_vms};
use std::collections::HashMap;

pub fn coordinate_routing_and_balancing(
    data_packet: Vec<u8>,
    routing_algorithm: RoutingAlgorithm,
) -> Result<(), String> {
    // List available VMs
    let available_vms = list_vms().map_err(|e| format!("Failed to list VMs: {:?}", e))?;


    if available_vms.is_empty() {
        return Err(String::from("No available VMs for routing"));
    }


    let mut vm_loads: HashMap<VMHandle, LoadMetric> = HashMap::new();
    for vm in &available_vms {
        let metrics = collect_metrics(vm).map_err(|e| format!("Failed to collect metrics: {:?}", e))?;
        let load_metric = calculate_load(metrics).map_err(|e| format!("Failed to calculate load: {:?}", e))?;
        vm_loads.insert(vm.clone(), load_metric);
    }

 
    let selected_vm = distribute_load(vm_loads, routing_algorithm)
        .map_err(|e| format!("Failed to distribute load: {:?}", e))?;

    let routing_criteria = RoutingCriteria {
        priority: 1,
        timestamp: 0, 
        task_type: String::from("default_task"),
    };


    route_data(data_packet, routing_criteria, selected_vm)
        .map_err(|e| format!("Failed to route data: {:?}", e))?;

    println!("Data successfully routed to VM: {:?}", selected_vm);
    Ok(())
}
