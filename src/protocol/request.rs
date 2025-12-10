use super::types::SOCKS5_VERSION;
use crate::error::ProtocolError;
use tokio::io::{AsyncRead, AsyncReadExt};

#[derive(Debug)]
pub struct ClientGreeting {
    pub version: u8,
    pub methods: Vec<u8>,
}

impl ClientGreeting {
    pub async fn read_from<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self, ProtocolError> {
        let version = reader.read_u8().await?;
        if version != SOCKS5_VERSION {
            return Err(ProtocolError::InvalidVersion(version));
        }

        let nmethods = reader.read_u8().await? as usize;
        let mut methods = vec![0u8; nmethods];
        reader.read_exact(&mut methods).await?;

        Ok(ClientGreeting { version, methods })
    }
}
