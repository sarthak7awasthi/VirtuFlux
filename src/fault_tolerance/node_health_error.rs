#[derive(Debug)]
pub enum NodeHealthError {
    UnreachableNode(String, String), 
}
