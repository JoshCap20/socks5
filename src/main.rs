use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;

use socks5::protocol::SOCKS5_DEFAULT_PORT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", SOCKS5_DEFAULT_PORT);
    let listener = TcpListener::bind(&addr).await?;
    println!("SOCKS5 server listening on {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            match socket.read(&mut buf).await {
                Ok(0) => return, // Connection closed
                Ok(n) => {
                    println!("Received {} bytes: {:02X?}", n, &buf[..n]);
                    // TODO: Parse greeting, do auth, handle request
                    // Connection closed for now
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                }
            }
        });
    }
}
