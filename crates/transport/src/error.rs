/// Common error type for transport operations.
pub type TransportResult<T> = Result<T, TransportError>;

/// Enum representing common errors that can occur during transport operations.
pub enum TransportError {}
