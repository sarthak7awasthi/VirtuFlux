use super::{VMError, VMHandle};
use std::process::Command;

#[derive(Debug)]
pub struct VMInfo {
    pub vm_id: String,
    pub state: String,
   
}

pub fn list_vms() -> Result<Vec<VMInfo>, VMError> {

    let list_command = Command::new("virsh")
        .arg("list")
        .arg("--all")
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !list_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&list_command.stderr).to_string(),
        ));
    }


    let output = String::from_utf8_lossy(&list_command.stdout);
    let mut vms = Vec::new();
    
    
    for line in output.lines().skip(2) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            vms.push(VMInfo {
                vm_id: parts[1].to_string(),
                state: parts[2..].join(" "),  
            });
        }
    }

    Ok(vms)
}
