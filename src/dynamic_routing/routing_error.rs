#[derive(Debug)]
pub enum RoutingError {
    NoAvailableVM,
    EmptyPacket,
    NetworkFailure,
}
