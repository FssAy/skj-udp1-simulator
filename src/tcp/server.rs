use std::net::{SocketAddr};
use std::str::FromStr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener};
use tokio::net::tcp::OwnedWriteHalf;
use crate::Config;


/// Creates a TCP server simulating SKJ's TCP server behaviour.
///
/// Returns UDP socket address received from the client.
/// Returns an Error on any communication issue.
///
/// If wrong flag or UDP address was provided, it just loops back and waits for another client.
/// This way you don't have to restart the server if something went wrong.
///
/// # Arguments
/// * `config` - A Config structure
///
pub async fn server(config: Config) -> Result<SocketAddr, tokio::io::Error> {
    let listener = TcpListener::bind(&config.tcp_address).await?;

    info!("TCP Server: ONLINE ON [{}], waiting for a client", config.tcp_address);

    loop {
        let (stream, address) = listener.accept().await?;
        info!("TCP Server: client [{}] connected, waiting for flag", address);

        let (stream_read, stream_write) = stream.into_split();
        let mut reader = BufReader::new(stream_read);
        let mut line = String::new();

        reader.read_line(&mut line).await?;

        line.pop();
        if line != config.init_flag.to_string() {
            warn!("TCP Server: invalid flag {:?}", line);
            kick(stream_write, &address).await;
            continue;
        }
        line.clear();

        info!("TCP Server: waiting for UDP address");
        reader.read_line(&mut line).await?;
        line.pop();

        let udp_address = match SocketAddr::from_str(&line) {
            Ok(udp_address) => udp_address,
            Err(error) => {
                warn!("TCP Server: invalid UDP address. {}", error);
                kick(stream_write, &address).await;
                continue;
            }
        };

        info!("TCP Server: initialization complete, shutting down");
        return Ok(udp_address);
    }
}

/// Shuts down connection with the client
async fn kick(mut stream: OwnedWriteHalf, address: &SocketAddr) {
    info!("TCP Server: client [{}] has been kicked", address);
    stream.shutdown().await.unwrap();
}
