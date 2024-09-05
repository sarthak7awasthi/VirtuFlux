use super::{VMHandle, VMError};
use std::process::Command;

pub fn stop_vm(vm_handle: &VMHandle) -> Result<(), VMError> {

    let stop_command = Command::new("virsh")
        .arg("shutdown")
        .arg(&vm_handle.vm_id)
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !stop_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&stop_command.stderr).to_string(),
        ));
    }

   
    Ok(())
}
