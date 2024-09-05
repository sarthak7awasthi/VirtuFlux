

use crate::fault_tolerance::node_status::check_node_status;
use crate::fault_tolerance::node_health_status::NodeHealthStatus;
use crate::fault_tolerance::node_health_error::NodeHealthError;
use crate::virtualization_management::vm_handle::VMHandle;

pub fn detect_node_failure(vm: &VMHandle, timeout_threshold: u64) -> Result<NodeHealthStatus, NodeHealthError> {
  
    let node_status = check_node_status(vm);

    
    if node_status.network_activity.response_time > timeout_threshold {
        println!("Node {} exceeded timeout threshold: {} ms", vm.vm_id, timeout_threshold);
        return Ok(NodeHealthStatus {
            vm_id: vm.vm_id.clone(),
            is_healthy: false,
            last_response_time: node_status.network_activity.response_time,
        });
    }

    if !node_status.is_healthy {
        println!("Node {} is unhealthy based on system metrics.", vm.vm_id);
        return Ok(NodeHealthStatus {
            vm_id: vm.vm_id.clone(),
            is_healthy: false,
            last_response_time: node_status.network_activity.response_time,
        });
    }

    Ok(NodeHealthStatus {
        vm_id: vm.vm_id.clone(),
        is_healthy: true,
        last_response_time: node_status.network_activity.response_time,
    })
}
