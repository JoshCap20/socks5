use tokio::net::TcpListener;

use socks5::protocol::{ClientGreeting, SOCKS5_DEFAULT_PORT};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("127.0.0.1:{}", SOCKS5_DEFAULT_PORT);
    let listener = TcpListener::bind(&addr).await?;
    println!("SOCKS5 server listening on {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("New connection from {}", addr);

        tokio::spawn(async move {
            match ClientGreeting::read_from(&mut socket).await {
                Ok(greeting) => {
                    println!("Client greeting: {:?}", greeting);
                    // TODO: Send server choice, handle auth, parse request
                }
                Err(e) => {
                    eprintln!("Failed to parse greeting: {}", e);
                }
            }
        });
    }
}
