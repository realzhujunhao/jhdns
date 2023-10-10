use std::{io::{self, Write}, net::TcpStream};

use clap::Parser;
use init::Args;
use protocol::{generate_request, read_response};

use crate::protocol::parse_response;

pub mod init;
pub mod protocol;

pub fn query() -> io::Result<()> {
    let args = Args::parse();
    let mut stream = TcpStream::connect(format!("{}:53", &args.dns_server_ip))?;
    
    let request = generate_request(&args.domain_name);

    stream.write_all(&request)?;
    let resp = read_response(&mut stream)?;

    println!("last 4 bytes(IPv4 on success)\n{:?}", parse_response(&resp));
    Ok(())
}
