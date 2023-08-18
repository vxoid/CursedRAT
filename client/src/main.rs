mod client;

use clap::Parser;
use client::Client;

#[derive(Clone, Parser)]
struct AppArgs {
  #[arg(short='s', long)]
  host: String,
}

fn main() {
  let args = AppArgs::parse();

  let server = args.host.parse().expect("Parsing error");
  let mut client = Client::connect(server).expect("Error creating client");

  loop {
    let command = client.wait_for_command();
    let command = match command {
      Ok(command) => command,
      Err(err) => {
        println!("error fetching a command: {err}");
        continue
      },
    };

    println!("{command}");
  }
}
