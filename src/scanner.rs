src/scanner.rs
(Port scanning logic – Govinda’s part)

use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Returns true if the port is open
pub fn scan_port(host: &str, port: u16, timeout_ms: u64) -> bool {
    let address = format!("{}:{}", host, port);
    let timeout = Duration::from_millis(timeout_ms);

    match address.to_socket_addrs() {
        Ok(mut addrs) => {
            if let Some(addr) = addrs.next() {
                TcpStream::connect_timeout(&addr, timeout).is_ok()
            } else {
                false
            }
        }
        Err(_) => false,
    }
}