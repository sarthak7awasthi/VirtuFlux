#[derive(Debug, Clone)]
pub struct NodeHealthStatus {
    pub vm_id: String,           
    pub is_healthy: bool,        
    pub last_response_time: u64, 
}
