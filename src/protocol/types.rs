pub const SOCKS5_DEFAULT_PORT: u16 = 1080;

pub const SOCKS5_VERSION: u8 = 0x05;

pub const RESERVED_BYTE: u8 = 0x00;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AuthMethodCode {
    NoAuth = 0x00,
    Gssapi = 0x01,
    UsernamePassword = 0x02,
    // 0x03-0x7F: IANA assigned
    // 0x80-0xFE: Reserved for private methods
    NoAcceptable = 0xFF,
}

impl TryFrom<u8> for AuthMethodCode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::NoAuth),
            0x01 => Ok(Self::Gssapi),
            0x02 => Ok(Self::UsernamePassword),
            0xFF => Ok(Self::NoAcceptable),
            other => Err(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    Connect = 0x01,
    Bind = 0x02,
    UdpAssociate = 0x03,
}

impl TryFrom<u8> for Command {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Connect),
            0x02 => Ok(Self::Bind),
            0x03 => Ok(Self::UdpAssociate),
            other => Err(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AddressType {
    IPv4 = 0x01, // 4 bytes
    Domain = 0x03, // 1 byte length + name
    IPv6 = 0x04, // 16 bytes
}

impl TryFrom<u8> for AddressType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::IPv4),
            0x03 => Ok(Self::Domain),
            0x04 => Ok(Self::IPv6),
            other => Err(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ReplyCode {
    Succeeded = 0x00,
    GeneralFailure = 0x01,
    ConnectionNotAllowed = 0x02,
    NetworkUnreachable = 0x03,
    HostUnreachable = 0x04,
    ConnectionRefused = 0x05,
    TtlExpired = 0x06,
    CommandNotSupported = 0x07,
    AddressTypeNotSupported = 0x08,
}

impl ReplyCode {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}