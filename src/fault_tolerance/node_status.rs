use crate::virtualization_management::vm_handle::VMHandle;
use sysinfo::{System, SystemExt, CpuExt, NetworkExt};


pub struct NetworkActivity {
    pub is_active: bool,
    pub response_time: u64, 
}


pub struct NodeStatus {
    pub vm_id: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub network_activity: NetworkActivity,
    pub is_healthy: bool,    
}

pub fn check_cpu_usage(vm: &VMHandle) -> f32 {
    let mut sys = System::new_all();
    sys.refresh_cpu();

 
    let cpu_usage = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    println!("Checked CPU usage for VM: {} -> {}%", vm.vm_id, cpu_usage);
    cpu_usage
}

pub fn check_memory_usage(vm: &VMHandle) -> f32 {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total_memory = sys.total_memory() as f32;
    let used_memory = sys.used_memory() as f32;

  
    let memory_usage_percentage = (used_memory / total_memory) * 100.0;

    println!("Checked memory usage for VM: {} -> {}%", vm.vm_id, memory_usage_percentage);
    memory_usage_percentage
}


pub fn check_network_activity(vm: &VMHandle) -> NetworkActivity {
    let mut sys = System::new_all();
    sys.refresh_networks();

    
    let network_data = sys.networks();
    let mut total_received = 0;
    let mut total_transmitted = 0;

    for (interface_name, data) in network_data.iter() {
        total_received += data.received();
        total_transmitted += data.transmitted();
        println!("Interface: {}, Received: {} bytes, Transmitted: {} bytes", interface_name, data.received(), data.transmitted());
    }

  
    let is_active = total_received > 0 && total_transmitted > 0;
    let response_time = 50; 

    NetworkActivity {
        is_active,
        response_time,
    }
}


pub fn check_node_status(vm: &VMHandle) -> NodeStatus {
  
    let cpu_usage = check_cpu_usage(vm);
    let memory_usage = check_memory_usage(vm);
    let network_activity = check_network_activity(vm);

    
    let is_healthy = cpu_usage < 80.0 && memory_usage < 80.0 && network_activity.is_active;


    NodeStatus {
        vm_id: vm.vm_id.clone(),
        cpu_usage,
        memory_usage,
        network_activity,
        is_healthy,
    }
}
