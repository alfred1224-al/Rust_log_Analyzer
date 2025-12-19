mod cli;
mod scanner;

use clap::Parser;
use cli::Args;
use scanner::scan_port;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn main() {
    let args = Args::parse();

    if args.start > args.end {
        eprintln!("❌ Start port must be less than or equal to end port");
        return;
    }

    println!("🔍 Rust Port Analyzer");
    println!(
        "Target: {} | Ports: {}-{} | Timeout: {}ms",
        args.host, args.start, args.end, args.timeout
    );

    let start_time = Instant::now();
    let open_ports = Arc::new(Mutex::new(Vec::new()));

    let ports: Vec<u16> = (args.start..=args.end).collect();
    let max_threads = 100;
    let mut handles = Vec::new();

    for chunk in ports.chunks(max_threads) {
        let host = args.host.clone();
        let timeout = args.timeout;
        let open_ports = Arc::clone(&open_ports);
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            for port in chunk {
                if scan_port(&host, port, timeout) {
                    println!("[+] Port {} is OPEN", port);
                    open_ports.lock().unwrap().push(port);
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start_time.elapsed();
    let open_ports = open_ports.lock().unwrap();

    println!("\n===== Scan Summary =====");
    println!("Total ports scanned: {}", args.end - args.start + 1);
    println!("Open ports found: {}", open_ports.len());

    if open_ports.is_empty() {
        println!("No open ports detected.");
    } else {
        println!("Open ports:");
        for port in open_ports.iter() {
            println!(" - {}", port);
        }
    }

    println!("Scan completed in {:.2?}", duration);
}
