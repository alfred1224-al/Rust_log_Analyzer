use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Rust TCP Port Analyzer")]
pub struct Args {
    /// Target host (IP or domain)
    #[arg(short, long)]
    pub host: String,

    /// Start port
    #[arg(short, long)]
    pub start: u16,

    /// End port
    #[arg(short, long)]
    pub end: u16,

    /// Timeout in milliseconds
    #[arg(short, long, default_value = "1000")]
    pub timeout: u64,
}
