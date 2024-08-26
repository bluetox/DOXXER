use async_std::net::TcpStream;
use futures::future::join_all;
use std::net::SocketAddr;
use std::time::Duration;
use colored::*;
use std::time::Instant;

pub async fn is_port_open(ip: &str, ports: Vec<u16>, timeout_secs: u64) -> Vec<bool> {
    let mut tasks = vec![];
    let start_time = Instant::now();
    for &port in &ports {
        let ip = ip.to_string();
        let task = async move {
            match async_std::io::timeout(Duration::from_secs(timeout_secs), TcpStream::connect(SocketAddr::new(ip.parse().unwrap(), port))).await {
                Ok(_) => {
                    let message = format!("Port {} is open", port);
                    println!("{}", message.truecolor(128, 0, 128));
                    true
                }
                Err(_) => false,
            }
        };
        tasks.push(task);
    }
    let results = join_all(tasks).await;

    let end_time = Instant::now();
    let duration = end_time - start_time;
    let num_ports = ports.len();
    println!("Scanned {} ports in {} milliseconds", num_ports, duration.as_millis());

    results
}
