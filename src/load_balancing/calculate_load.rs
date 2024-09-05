use crate::virtualization_management::vm_handle::VMHandle;
use crate::load_balancing::load_metric::LoadMetric;
use sysinfo::{System, SystemExt, ProcessExt};


pub fn calculate_load(vm_handle: &VMHandle) -> LoadMetric {
    let mut sys = System::new_all();

 
    sys.refresh_all();


    let cpu_usage = get_cpu_usage(vm_handle, &sys);
    let memory_usage = get_memory_usage(vm_handle, &sys);

   
    LoadMetric {
        cpu_usage,
        memory_usage,
        total_load: (cpu_usage + (memory_usage as f32) / 1024.0) / 2.0,  
    }
}


fn get_cpu_usage(vm_handle: &VMHandle, sys: &System) -> f32 {

    if let Some(process) = sys.process_by_name(&vm_handle.vm_id).first() {
        return process.cpu_usage();  
    }

    
    0.0
}


fn get_memory_usage(vm_handle: &VMHandle, sys: &System) -> u32 {
 
    if let Some(process) = sys.process_by_name(&vm_handle.vm_id).first() {
        return (process.memory() / 1024) as u32; 
    }

   
    0
}
