

use crate::virtualization_management::vm_handle::VMHandle;
use crate::load_balancing::distribute_load;
use crate::fault_tolerance::node_health_error::NodeHealthError;

pub fn initiate_failover(
    failed_vm: &VMHandle, 
    healthy_vms: Vec<VMHandle>, 
    failed_tasks: Vec<Vec<u8>>, 
) -> Result<(), NodeHealthError> {
    if healthy_vms.is_empty() {
        return Err(NodeHealthError::NoHealthyNodes);
    }

   
    let redistributed_vms = distribute_load(failed_tasks, healthy_vms.clone());

 
    for vm in &redistributed_vms {
        println!("Task redistributed to VM: {}", vm.vm_id);
    }

    Ok(())
}
