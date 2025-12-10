use thiserror::Error;
use std::io;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("invalid SOCKS version: expected 5, got {0}")]
    InvalidVersion(u8),

    #[error("invalid command: 0x{0:02X}")]
    InvalidCommand(u8),

    #[error("invalid address type: 0x{0:02X}")]
    InvalidAddressType(u8),

    #[error("domain name too long: {0} bytes (max 255)")]
    DomainTooLong(usize),

    #[error("unexpected end of data")]
    UnexpectedEof,

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("no acceptable authentication method")]
    NoAcceptableMethod,

    #[error("authentication failed: {0}")]
    Failed(String),

    #[error("invalid credentials format")]
    InvalidFormat,

    #[error("authentication timeout")]
    Timeout,

    #[error("I/O error during auth: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("target connection refused")]
    ConnectionRefused,

    #[error("network unreachable")]
    NetworkUnreachable,

    #[error("host unreachable")]
    HostUnreachable,

    #[error("connection timeout")]
    Timeout,

    #[error("DNS resolution failed: {0}")]
    DnsFailure(String),

    #[error("command not supported: {0:?}")]
    NotSupported(crate::protocol::types::Command),

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("connection blocked by IP filter")]
    IpBlocked,

    #[error("domain blocked: {0}")]
    DomainBlocked(String),

    #[error("rate limit exceeded")]
    RateLimitExceeded,

    #[error("connection limit exceeded")]
    ConnectionLimitExceeded,

    #[error("max connections per IP exceeded")]
    PerIpLimitExceeded,

    #[error("max connections per user exceeded")]
    PerUserLimitExceeded,
}