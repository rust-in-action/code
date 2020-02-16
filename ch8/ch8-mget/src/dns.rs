use std::error::Error;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::proto::error::ProtoError;
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

fn message_id() -> u16 {
  let candidate = rand::random();
  if candidate == 0 {
    return message_id();
  }
  candidate
}

#[derive(Debug)]
pub enum DnsError {
  ParseDomainName(ProtoError),
  ParseDnsServerAddress(std::net::AddrParseError),
  Encoding(ProtoError),
  Decoding(ProtoError),
  Network(std::io::Error),
  Sending(std::io::Error),
  Receving(std::io::Error),
}

impl std::fmt::Display for DnsError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{:#?}", self)
  }
}

impl std::error::Error for DnsError {
  // use default methods
}

pub fn resolve(dns_server_address: &str, domain_name: &str) -> Result<Option<std::net::IpAddr>, Box<Error>> {
  // input parsing
  let domain_name = Name::from_ascii(domain_name).map_err(DnsError::ParseDomainName)?;
  let dns_server_address = format!("{}:53", dns_server_address);
  let dns_server: SocketAddr = dns_server_address.parse().map_err(DnsError::ParseDnsServerAddress)?;

  // allocate buffers
  let mut request_buffer: Vec<u8> = Vec::with_capacity(50);
  let mut response_buffer: [u8; 512] = [0; 512]; // DNS over UDP uses a maximum packet size of 512 bytes.

  let mut request = Message::new();
  // let ipv4_query = ;
  request.add_query(Query::query(domain_name, RecordType::A)); // DNS messages can hold multiple queries, but here we're only using a single one
  request
    .set_id(message_id())
    .set_message_type(MessageType::Query)
    .set_op_code(OpCode::Query)
    .set_recursion_desired(true); // ask the DNS server that we're connecting to to make requests on our behalf if it doesn't know the answer

  let timeout = Duration::from_secs(5);
  let localhost = UdpSocket::bind("0.0.0.0:0").map_err(DnsError::Network)?; // Binding to port 0 asks the operating system to allocate a port on our behalf
  localhost.set_read_timeout(Some(timeout)).map_err(DnsError::Network)?;
  localhost.set_nonblocking(false).map_err(DnsError::Network)?;

  let mut encoder = BinEncoder::new(&mut request_buffer);
  request.emit(&mut encoder).map_err(DnsError::Encoding)?;

  let _n_bytes_sent = localhost
    .send_to(&request_buffer, dns_server)
    .map_err(DnsError::Sending)?;

  // There is a minuscule chance that another UDP message will be received on our
  // port from some unknown sender. To avoid that, we ignore packets from IP addresses that we don't expect.
  loop {
    let (_b_bytes_recv, remote_port) = localhost.recv_from(&mut response_buffer).map_err(DnsError::Receving)?;
    if remote_port == dns_server {
      break;
    }
  }

  let response = Message::from_vec(&response_buffer).map_err(DnsError::Decoding)?;
  for answer in response.answers() {
    if answer.record_type() == RecordType::A {
      let resource = answer.rdata();
      let server_ip = resource.to_ip_addr().expect("invalid IP address received");
      return Ok(Some(server_ip));
    }
  }

  Ok(None)
}
