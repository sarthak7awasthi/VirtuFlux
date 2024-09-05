use crate::virtualization_management::vm_handle::VMHandle;
use crate::load_balancing::load_metric::LoadMetric;


pub struct LoadDistributionMetrics {
    pub average_cpu_usage: f32,
    pub average_memory_usage: u32,
    pub total_tasks: usize,
    pub task_distribution: Vec<(String, usize)>, 
}


pub fn monitor_load_distribution(
    vm_loads: Vec<(VMHandle, LoadMetric)>,
    task_distribution: Vec<(String, usize)>,
) -> LoadDistributionMetrics {
    let mut total_cpu_usage = 0.0;
    let mut total_memory_usage = 0;
    let mut total_tasks = 0;

    for (vm, load) in &vm_loads {
        total_cpu_usage += load.cpu_usage;
        total_memory_usage += load.memory_usage;
    }

    for (_, tasks) in &task_distribution {
        total_tasks += tasks;
    }

   
    let average_cpu_usage = total_cpu_usage / vm_loads.len() as f32;
    let average_memory_usage = total_memory_usage / vm_loads.len() as u32;

    LoadDistributionMetrics {
        average_cpu_usage,
        average_memory_usage,
        total_tasks,
        task_distribution,
    }
}
