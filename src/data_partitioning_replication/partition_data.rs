use crate::data_partitioning_replication::partition_strategy::PartitionStrategy;
use crate::data_partitioning_replication::partition_error::PartitionError;

#[derive(Debug, Clone)]
pub struct DataChunk {
    pub chunk_id: String,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct LargeDataset {
    pub dataset_id: String,
    pub data: Vec<u8>,
}

pub fn partition_data(
    dataset: LargeDataset,
    strategy: PartitionStrategy,
) -> Result<Vec<DataChunk>, PartitionError> {
    match strategy {
        PartitionStrategy::BySize(chunk_size) => {
            let mut chunks = Vec::new();
            let mut chunk_id = 1;
            for data_chunk in dataset.data.chunks(chunk_size) {
                chunks.push(DataChunk {
                    chunk_id: format!("chunk_{}", chunk_id),
                    data: data_chunk.to_vec(),
                });
                chunk_id += 1;
            }
            Ok(chunks)
        },
        PartitionStrategy::RoundRobin(num_chunks) => {
            let chunk_size = (dataset.data.len() as f32 / num_chunks as f32).ceil() as usize;
            let mut chunks = Vec::new();
            let mut chunk_id = 1;
            for data_chunk in dataset.data.chunks(chunk_size) {
                chunks.push(DataChunk {
                    chunk_id: format!("chunk_{}", chunk_id),
                    data: data_chunk.to_vec(),
                });
                chunk_id += 1;
            }
            Ok(chunks)
        },
    }
}
