use super::{VMHandle, NetworkConfig, VMError};
use std::process::Command;

pub fn create_vm(
    vm_id: String,
    cpu_cores: u8,
    memory_mb: u32,
    disk_size_gb: u32,
    network_config: NetworkConfig,
) -> Result<VMHandle, VMError> {
 
    let create_command = Command::new("qemu-system-x86_64")
        .arg("-enable-kvm")
        .arg(format!("-smp {}", cpu_cores))
        .arg(format!("-m {}", memory_mb))
        .arg(format!("-hda /var/lib/libvirt/images/{}.img", vm_id))
        .arg(format!("-net nic -net user,hostname={}", network_config.ip_address))
        .arg("-daemonize")
        .output()
        .map_err(|e| VMError::CreationFailed(e.to_string()))?;

    if !create_command.status.success() {
        return Err(VMError::CreationFailed(
            String::from_utf8_lossy(&create_command.stderr).to_string(),
        ));
    }

  
    Ok(VMHandle { vm_id })
}
