# Rust_port_scanner
Rust Group project

RUST PORT SCANNER

This is a simple, multi-threaded TCP port scanner written in Rust. It was built to learn how Rust handles networking and concurrency. It checks for open ports by attempting to establish a TCP connection and reporting which ones respond.

🚀 Overview
This scanner allows users to identify open ports on a target IP address. By leveraging Multi-Producer, Single-Consumer (MPSC) channels and thread spawning, it can scan all 65,535 ports in a fraction of the time required by a single-threaded scanner.

Key Technical Features

Concurrency: Uses std::thread to avoid blocking the main execution.
MPSC Channels: Implements std::sync::mpsc for safe communication between worker threads and the main aggregator.
Low Overhead: Built using only the Rust standard library—no heavy external dependencies.
Adaptive Throttling: Users can define the number of threads (-j flag) to balance speed and system resources.

🛠️ How It Works
Input Parsing: The program captures the target IP and the desired thread count from the command line.
Thread Dispatch: The port range (0–65535) is divided among the specified number of threads.
Connection Attempt: Each thread attempts to open a TcpStream to its assigned ports.
success: If the port is open, it sends the port number through the Sender channel.
Failure: If the connection is refused or times out, it moves to the next port.
Aggregation: The main thread (the Receiver) collects all successful port numbers, sorts them, and displays the final list.

💻 Usage
Prerequisites
Rust/Cargo (Latest stable version recommended)

Build and Run
Clone the repository and build the binary:
git clone https://github.com/alfred1224-al/Rust_port_scanner.git
cd Rust_port_scanner
cargo build --release

To run the scanner, use the following syntax:
# General Syntax
cargo run -- <TARGET_IP> -j <THREADS>
# Example: Scan your local router with 100 threads
cargo run -- 192.168.1.1 -j 100

📊 Performance Comparison

Mode,Estimated                  Time (Standard)                   Stability

Single-threaded                  10-20 minutes                    Very High
Multi-threaded(100 threads)      30-60 seconds                      High
Multi-threaded(500+ threads)     10-15 seconds                    Moderate (Network Dependent)

Note:
This tool is for educational use only. Only scan networks and devices you own or have permission to test.




