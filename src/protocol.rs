use std::{
    io::{self, Read},
    net::TcpStream,
};

pub fn generate_request(domain_name: &str) -> Vec<u8> {
    let mut request = Vec::new();
    let trans_id = rand::random::<[u8; 2]>();
    let flags = [1, 0];
    let questions = [0, 1];
    let rr = [0, 0, 0, 0, 0, 0];

    request.extend_from_slice(&trans_id);
    request.extend_from_slice(&flags);
    request.extend_from_slice(&questions);
    request.extend_from_slice(&rr);

    for level in domain_name.split('.') {
        request.push(level.len() as u8);
        request.extend_from_slice(level.as_bytes());
    }
    request.push(0);

    let type_a = [0, 1];
    let class_in = [0, 1];

    request.extend_from_slice(&type_a);
    request.extend_from_slice(&class_in);

    let mut wrap_request = Vec::new();
    wrap_request.extend_from_slice(&(request.len() as u16).to_be_bytes());
    wrap_request.extend_from_slice(&request);

    wrap_request
}

pub fn read_response(stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut len_buf = [0; 2];
    stream.read_exact(&mut len_buf)?;
    let len = u16::from_be_bytes(len_buf);
    println!("response lenth: {}", len);
    let mut resp_buf = vec![0; len as usize];
    stream.read_exact(&mut resp_buf)?;
    println!("response bytes:\n {:?}", resp_buf);
    Ok(resp_buf)
}

pub fn parse_response(resp: &[u8]) -> [u8; 4] { 
    let ip_start_idx = resp.len() - 4;
    let ip_end_idx = resp.len();
    let ip_address = &resp[ip_start_idx..ip_end_idx];
    [ip_address[0], ip_address[1], ip_address[2], ip_address[3]]
}
