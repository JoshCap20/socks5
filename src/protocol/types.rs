/// SOCKS5 protocol version
pub const SOCKS_VERSION: u8 = 0x05;

/// Reserved byte
pub const RSV: u8 = 0x00;

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