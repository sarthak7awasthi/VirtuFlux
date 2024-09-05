use super::{VMHandle, NetworkConfig, VMError};
use std::process::Command;

pub fn setup_vm_networking(vm_handle: &VMHandle, network_config: &NetworkConfig) -> Result<(), VMError> {
  
    let mac_address = get_vm_mac_address(&vm_handle.vm_id)?;

    let attach_network_command = Command::new("virsh")
        .arg("attach-interface")
        .arg(&vm_handle.vm_id)
        .arg("network")
        .arg("default")
        .arg("--model")
        .arg("virtio")
        .arg("--config")
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !attach_network_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&attach_network_command.stderr).to_string(),
        ));
    }

    
    let set_ip_command = Command::new("virsh")
        .arg("net-update")
        .arg("default")
        .arg("add")
        .arg("ip-dhcp-host")
        .arg(format!("--mac {} --ip {} --xml", mac_address, network_config.ip_address))
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !set_ip_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&set_ip_command.stderr).to_string(),
        ));
    }

    Ok(())
}


fn get_vm_mac_address(vm_id: &str) -> Result<String, VMError> {
    let output = Command::new("virsh")
        .arg("domiflist")
        .arg(vm_id)
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !output.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }


    let output_str = String::from_utf8_lossy(&output.stdout);
    for line in output_str.lines() {
        if line.contains("mac") {
            let mac = line.split_whitespace().nth(4);
            if let Some(mac_address) = mac {
                return Ok(mac_address.to_string());
            }
        }
    }

    Err(VMError::CreationFailed("Failed to retrieve MAC address".to_string()))
}
