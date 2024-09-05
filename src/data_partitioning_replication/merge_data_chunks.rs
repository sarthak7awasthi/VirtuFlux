use crate::data_partitioning_replication::partition_data::DataChunk;
use crate::data_partitioning_replication::partition_error::MergeError;

/// Merges the data chunks back into the original dataset.
pub fn merge_data_chunks(chunks: Vec<DataChunk>) -> Result<Vec<u8>, MergeError> {
  
    let mut sorted_chunks = chunks;
    sorted_chunks.sort_by(|a, b| a.chunk_id.cmp(&b.chunk_id));

  
    let mut merged_data: Vec<u8> = Vec::new();
    for chunk in sorted_chunks {
        merged_data.extend(chunk.data);
    }

    Ok(merged_data)
}
