use crate::virtualization_management::VMHandle;
use crate::dynamic_routing::{RoutingCriteria, RoutingConfig, RoutingError, monitor_routing_performance::RoutingPerformanceMonitor};
use std::time::Instant;

pub fn route_data(
    packet_id: &str,
    data_packet: Vec<u8>,
    routing_criteria: RoutingCriteria,
    available_vms: &[VMHandle],
    routing_config: &RoutingConfig,
    performance_monitor: &RoutingPerformanceMonitor, 
) -> Result<VMHandle, RoutingError> {
    if data_packet.is_empty() {
        return Err(RoutingError::EmptyPacket);
    }

  
    let start_time = Instant::now();

 
    let selected_vm = routing_config
        .select_vm(available_vms)
        .ok_or(RoutingError::NoAvailableVM)?;

  
    println!(
        "Routing packet {} to VM {} with criteria: priority = {}, processing_type = {}",
        packet_id, selected_vm.vm_id, routing_criteria.priority, routing_criteria.processing_type
    );

   
    let end_time = Instant::now();

   
    performance_monitor.update_metrics(data_packet.len(), start_time, end_time);


    Ok(selected_vm)
}
