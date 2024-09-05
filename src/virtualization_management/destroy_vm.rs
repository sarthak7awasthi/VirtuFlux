use super::{VMHandle, VMError};
use std::process::Command;

pub fn destroy_vm(vm_handle: &VMHandle) -> Result<(), VMError> {
   
    let destroy_command = Command::new("virsh")
        .arg("destroy")
        .arg(&vm_handle.vm_id)
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !destroy_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&destroy_command.stderr).to_string(),
        ));
    }


    Ok(())
}
