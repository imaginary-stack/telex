use clap::Command;
use network::receive::listener;

mod message;
mod network;
mod config;

fn main() {
    let app = Command::new("telex")
        .about("for transferring messages within a LAN")
        .version("0.1.0")
        .subcommand_required(true);
}
