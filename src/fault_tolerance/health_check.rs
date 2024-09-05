
use crate::virtualization_management::vm_handle::VMHandle;
use crate::fault_tolerance::node_health_status::NodeHealthStatus;
use crate::fault_tolerance::node_status::{check_cpu_usage, check_memory_usage, check_network_activity};

pub fn health_check(vm: &VMHandle) -> NodeHealthStatus {
    let cpu_usage = check_cpu_usage(vm);
    let memory_usage = check_memory_usage(vm);
    let network_activity = check_network_activity(vm);

   
    let is_healthy = cpu_usage < 80.0 && memory_usage < 80.0 && network_activity.is_active;

    NodeHealthStatus {
        vm_id: vm.vm_id.clone(),
        is_healthy,
        last_response_time: network_activity.response_time, 
    }
}
