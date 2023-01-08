use network::receive::listener;

mod message;
mod network;

fn main() -> std::io::Result<()> {
    listener();
    Ok(())
}
