use network::listener;

mod message;
mod network;

fn main() -> std::io::Result<()> {
    listener();
    Ok(())
}
