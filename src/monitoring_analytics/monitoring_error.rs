#[derive(Debug)]
pub enum MonitoringError {
    VMNotFound(String),
    MetricsUnavailable(String),
}

impl std::fmt::Display for MonitoringError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MonitoringError::VMNotFound(ref vm_id) => write!(f, "VM with ID {} not found", vm_id),
            MonitoringError::MetricsUnavailable(ref vm_id) => write!(f, "Metrics unavailable for VM {}", vm_id),
        }
    }
}

impl std::error::Error for MonitoringError {}
