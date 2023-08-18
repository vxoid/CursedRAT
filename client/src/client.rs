use std::io::{self, Read};
use std::io::Write;
use std::net;

pub struct Client {
  connection: net::TcpStream,
  server: net::SocketAddr,
  id: u64
}

impl Client {
  pub fn connect(server: net::SocketAddr) -> io::Result<Self> {
    let mut connection = net::TcpStream::connect(server)?;

    let os = std::env::consts::OS;

    connection.write_all(os.as_bytes())?;

    let mut id_bytes = [0u8; 8];
    connection.read_exact(&mut id_bytes)?;

    let id = u64::from_be_bytes(id_bytes);
    
    Ok(Self { connection, server, id })
  }

  pub fn wait_for_command(&mut self) -> io::Result<String> {
    let mut command = String::new();
    self.connection.read_to_string(&mut command)?;
    Ok(command)
  }
}