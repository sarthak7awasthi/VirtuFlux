use super::{VMHandle, VMError};
use std::process::Command;

pub fn start_vm(vm_handle: &VMHandle) -> Result<(), VMError> {
 
    let start_command = Command::new("virsh")
        .arg("start")
        .arg(&vm_handle.vm_id)
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !start_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&start_command.stderr).to_string(),
        ));
    }

  
    Ok(())
}
