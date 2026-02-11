use std::net::TcpStream;
use std::io::Write;
use crate::protocol::signed::SignedCommand;

pub fn send_signed_command(addr: &str, signed: &SignedCommand) -> std::io::Result<()> {
    let mut s = TcpStream::connect(addr)?;
    let txt = serde_json::to_string(&signed).unwrap();
    s.write_all(txt.as_bytes())?;
    s.flush()?; // ensure data is sent
    // drop connection immediately (no read blocking)
    Ok(())
}
