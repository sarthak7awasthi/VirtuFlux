use std::fmt;

#[derive(Debug, Clone)]
pub enum PartitionError {
    EmptyDataset,
}

impl fmt::Display for PartitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PartitionError::EmptyDataset => write!(f, "Dataset is empty and cannot be partitioned."),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReplicationError {
    InsufficientNodes,
}

impl fmt::Display for ReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplicationError::InsufficientNodes => write!(f, "Not enough nodes available for the specified replication factor."),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConsistencyError {
    InconsistentData,
}

impl fmt::Display for ConsistencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConsistencyError::InconsistentData => write!(f, "Data chunks are inconsistent across VMs."),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MergeError {
    EmptyChunks,
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MergeError::EmptyChunks => write!(f, "Cannot merge data chunks because they are empty."),
        }
    }
}
