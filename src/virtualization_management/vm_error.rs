#[derive(Debug)]
pub enum VMError {
    CreationFailed(String),
    InvalidConfiguration(String),
   
}
