#[macro_use] extern crate tracing;

mod tcp;
mod udp;
mod config;

use tracing::{error, Level};
use tracing::subscriber::set_global_default;
use tracing_subscriber::FmtSubscriber;
use config::Config;

// Do not change these constants
pub(crate) const NUM_MIN: u64 = 1000000000;
pub(crate) const NUM_MAX: u64 = 9999999999;
pub(crate) const TASKS_AMOUNT: u8 = 3;


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .without_time()
        .with_max_level(Level::INFO)
        .with_ansi(ansi_term::enable_ansi_support().is_ok())
        .finish();

    set_global_default(subscriber).unwrap();

    info!("UDP - Prog 3 PL - SIMULATOR ver.[{}]", env!("CARGO_PKG_VERSION"));
    info!("Author: FssAy#8648");
    println!();

    let config = match Config::load() {
        Ok(config) => {
            info!("Config has been loaded.");
            info!("[TCP]: {}", config.tcp_address);
            info!("[UDP]: {}", config.udp_address);
            info!("[Flag]: {}", config.init_flag);
            info!("[Seed]: {}", config.seed);
            for task in config.get_tasks() {
                info!("[--Task--]: {}", task);
            }
            config
        }
        Err(message) => {
            error!("Couldn't load the config [{}]", message);
            return;
        }
    };

    println!();

    let udp_address = match tcp::server(config.clone()).await {
        Ok(udp_address) => udp_address,
        Err(error) => {
            error!("TCP Server: {}", error);
            return;
        }
    };

    println!();

    if let Err(error) = udp::server(config.clone(), udp_address).await {
        error!("UDP Server: {}", error);
    } else {
        println!();
        info!("Simulation complete!");
    }
}
