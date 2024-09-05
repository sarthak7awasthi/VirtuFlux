use crate::data_partitioning_replication::partition_data::DataChunk;
use crate::data_partitioning_replication::partition_error::ConsistencyError;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReplicatedChunk {
    pub vm_id: String,
    pub chunk: DataChunk,
}

pub fn replicate_data(
    data_chunks: Vec<DataChunk>,
    replication_factor: u8,
    available_vms: Vec<String>,
) -> Result<Vec<ReplicatedChunk>, ReplicationError> {
 
    if available_vms.len() < replication_factor as usize {
        return Err(ReplicationError::InsufficientNodes);
    }

    let mut replicated_chunks = Vec::new();
    
    // Iterate over the data chunks and replicate them across the VMs
    for chunk in data_chunks {
    
        for vm_id in available_vms.iter().take(replication_factor as usize) {
            replicated_chunks.push(ReplicatedChunk {
                vm_id: vm_id.clone(),
                chunk: chunk.clone(),
            });
        }
    }

    Ok(replicated_chunks)
}

pub fn check_data_consistency(
	replicated_data: Vec<ReplicatedChunk>
) -> Result<bool, ConsistencyError> {
	let mut chunk_consistency_map: HashMap<String, Vec<Vec<u8>>> = HashMap::new();


	for replicated_chunk in replicated_data {
			let chunk_data = replicated_chunk.chunk.data.clone();
			let chunk_entry = chunk_consistency_map.entry(replicated_chunk.chunk.chunk_id.clone()).or_insert(Vec::new());
			chunk_entry.push(chunk_data);
	}


	for (_chunk_id, data_list) in chunk_consistency_map.iter() {
			let first_data = &data_list[0];
			for data in data_list.iter() {
					if data != first_data {
							return Err(ConsistencyError::InconsistentData);
					}
			}
	}

	Ok(true)
}