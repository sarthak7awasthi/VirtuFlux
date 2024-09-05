use crate::virtualization_management::vm_handle::VMHandle;
use crate::load_balancing::calculate_load;
use crate::load_balancing::load_metric::LoadMetric;


pub fn distribute_load(data_packets: Vec<Vec<u8>>, available_vms: Vec<VMHandle>) -> Vec<VMHandle> {
    let mut vm_loads: Vec<(VMHandle, LoadMetric)> = available_vms
        .into_iter()
        .map(|vm| {
            let load = calculate_load(&vm);
            (vm, load)
        })
        .collect();


    vm_loads.sort_by(|a, b| a.1.total_load.partial_cmp(&b.1.total_load).unwrap());

    let mut routed_vms = Vec::new();

  
    for packet in data_packets {
        if let Some((vm, _)) = vm_loads.get_mut(0) {
       
            println!(
                "Routing packet to VM {} with current load {:.2}%",
                vm.vm_id, vm_loads[0].1.total_load
            );
            routed_vms.push(vm.clone());

         
            vm_loads[0].1.total_load += 5.0; 

            vm_loads.sort_by(|a, b| a.1.total_load.partial_cmp(&b.1.total_load).unwrap());
        }
    }

    routed_vms
}
