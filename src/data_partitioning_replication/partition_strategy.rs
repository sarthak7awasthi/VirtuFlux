#[derive(Debug, Clone)]
pub enum PartitionStrategy {
    BySize(usize),      // Partition based on the chunk size in bytes
    RoundRobin(usize),  // Partition into a fixed number of chunks
}
