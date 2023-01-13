use clap::Command;
use config::Config;

mod config;
mod message;
mod network;

fn main() {
    let config = Config::read_file();

    let app = Command::new("telex")
        .about("for transferring messages within a LAN")
        .version("0.1.0")
        .subcommand_required(true);
}
