use crate::virtualization_management::{
	create_vm, start_vm, stop_vm, destroy_vm, list_vms, NetworkConfig, VMHandle,
};
use std::collections::HashMap;

pub enum VMAction {
	Create {
			vm_id: String,
			cpu_cores: u8,
			memory_mb: u32,
			disk_size_gb: u32,
			network_config: NetworkConfig,
	},
	Start {
			vm_handle: VMHandle,
	},
	Stop {
			vm_handle: VMHandle,
	},
	Destroy {
			vm_handle: VMHandle,
	},
	List,
}

pub fn manage_vm_lifecycle(action: VMAction) -> Result<(), String> {
	match action {
			VMAction::Create {
					vm_id,
					cpu_cores,
					memory_mb,
					disk_size_gb,
					network_config,
			} => {
					let vm_handle = create_vm(vm_id, cpu_cores, memory_mb, disk_size_gb, network_config)
							.map_err(|e| format!("Failed to create VM: {:?}", e))?;
					println!("VM created with handle: {:?}", vm_handle);
			}
			VMAction::Start { vm_handle } => {
					start_vm(&vm_handle).map_err(|e| format!("Failed to start VM: {:?}", e))?;
					println!("VM started with handle: {:?}", vm_handle);
			}
			VMAction::Stop { vm_handle } => {
					stop_vm(&vm_handle).map_err(|e| format!("Failed to stop VM: {:?}", e))?;
					println!("VM stopped with handle: {:?}", vm_handle);
			}
			VMAction::Destroy { vm_handle } => {
					destroy_vm(&vm_handle).map_err(|e| format!("Failed to destroy VM: {:?}", e))?;
					println!("VM destroyed with handle: {:?}", vm_handle);
			}
			VMAction::List => {
					let vms = list_vms().map_err(|e| format!("Failed to list VMs: {:?}", e))?;
					println!("Available VMs: {:?}", vms);
			}
	}
	Ok(())
}
