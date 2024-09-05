use crate::virtualization_management::vm_handle::VMHandle;
use std::collections::HashMap;

pub fn recover_data(
    data_id: &str,
    replicas: &HashMap<String, Vec<u8>>,
    healthy_vms: Vec<VMHandle>,      
) -> Result<Vec<u8>, String> {
    for vm in &healthy_vms {
        if let Some(data_chunk) = replicas.get(&vm.vm_id) {
            println!("Recovered data from VM: {}", vm.vm_id);
            return Ok(data_chunk.clone());
        }
    }

    Err(format!("Failed to recover data for ID: {}. No replicas found.", data_id))
}
