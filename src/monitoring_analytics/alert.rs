use std::time::{SystemTime, UNIX_EPOCH};

pub enum AlertType {
    HighCpuUsage,
    HighMemoryUsage,
    HighNetworkUsage,
    NodeFailure,
}

pub struct Alert {
    pub alert_id: String,
    pub alert_type: AlertType,
    pub description: String,
    pub severity: u8,       
    pub timestamp: u64,    
}

impl Alert {
    pub fn new(alert_type: AlertType, description: String, severity: u8) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect("Time went backwards").as_secs();

        Self {
            alert_id: uuid::Uuid::new_v4().to_string(),  
            alert_type,
            description,
            severity,
            timestamp,
        }
    }
}
