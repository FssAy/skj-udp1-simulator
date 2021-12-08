use std::net::SocketAddr;
use tokio::net::{UdpSocket};
use crate::Config;
use crate::udp::Task;


fn packet<S: ToString>(data: S) -> Vec<u8> {
    format!("{}\n", data.to_string()).as_bytes().to_vec()
}

async fn recv(socket: &UdpSocket) -> Result<String, tokio::io::Error> {
    let mut data = String::new();
    let mut buffer = [0_u8; 1024];
    let mut size = buffer.len();
    while size == buffer.len() {
        if data.len() > buffer.len()*10 {
            error!("Exceeded max bytes to receive");
            return Ok(String::new());
        }
        size = socket.recv(&mut buffer).await?;
        data = format!("{}{}", data, String::from_utf8_lossy(&buffer[0..size]));
    }
    data.pop();
    Ok(data)
}

/// Creates a UDP server simulating SKJ's UDP server behaviour
///
/// # Arguments
/// * `config` - A Config structure
/// * `udp_address` - UDP client's address received from previous TCP connection
///
pub async fn server(config: Config, udp_address: SocketAddr) -> Result<(), tokio::io::Error> {
    let socket = UdpSocket::bind(&config.udp_address).await?;
    info!("UDP Server: ONLINE ON [{}]", config.udp_address);

    info!("UDP Server: Connecting to [{}]", udp_address);
    socket.connect(udp_address).await?;

    println!();

    let mut tasks_num = 0_u8;

    for task in config.get_tasks() {
        info!("[Task manager]: sending task data");

        match task {
            Task::GCD { numbers, result } => {
                for number in numbers {
                    socket.send_to(&packet(number), &udp_address).await?;
                }

                let data = recv(&socket).await?;
                if data == format!("{}", result) {
                    info!("[Task GCD]: completed!");
                } else {
                    error!("[Task GCD]: failed! Provided: [{}], Expected: [{}]", data, result);
                    break;
                }
            }
            Task::SUM { numbers, result } => {
                for number in numbers {
                    socket.send_to(&packet(number), &udp_address).await?;
                }

                let data = recv(&socket).await?;
                if data == format!("{}", result) {
                    info!("[Task SUM]: completed!");
                } else {
                    error!("[Task SUM]: failed! Provided: [{}], Expected: [{}]", data, result);
                    break;
                }
            }
            Task::XK { x, result, .. } => {
                socket.send_to(&packet(x), &udp_address).await?;

                let data = recv(&socket).await?;
                if data == format!("{}", result) {
                    info!("[Task XK]: completed!");
                } else {
                    error!("[Task XK]: failed! Provided: [{}], Expected: [{}]", data, result);
                    break;
                }
            }
            Task::StringDeletion { target, result, .. } => {
                socket.send_to(&packet(target), &udp_address).await?;

                let data = recv(&socket).await?;
                if data == format!("{}", result) {
                    info!("[Task SD]: completed!");
                } else {
                    error!("[Task SD]: failed! Provided: [{}], Expected: [{}]", data, result);
                    break;
                }
            }
            Task::StringConcat { target, result } => {
                socket.send_to(&packet(target), &udp_address).await?;

                let data = recv(&socket).await?;
                if data == format!("{}", result) {
                    info!("[Task SC]: completed!");
                } else {
                    error!("[Task SC]: failed! Provided: [{}], Expected: [{}]", data, result);
                    break;
                }
            }
        }

        tasks_num += 1;
    }

    if tasks_num >= crate::TASKS_AMOUNT {
        socket.send_to(&packet(config.final_flag), &udp_address).await?;
    }

    Ok(())
}
