extern crate log;
extern crate clap;
extern crate env_logger;
extern crate rand;
extern crate smoltcp;
extern crate url;

use clap::{App, Arg};
use url::Url;
use smoltcp::phy::TapInterface;

mod dns;
mod http;
mod ethernet;

fn main() {
  let app = App::new("mget")
      .about("GET a webpage, manually")
      .arg(Arg::with_name("url").required(true))
      .arg(Arg::with_name("tap-device").required(true))
      .arg(Arg::with_name("dns-server").short("s").default_value("1.1.1.1"))
      .get_matches();

  // read raw values from command-line
  let url_text = app.value_of("url").unwrap();
  let dns_server_text = app.value_of("dns-server").unwrap();
  let tap_text = app.value_of("tap-device").unwrap();

  let url = Url::parse(url_text).expect("unable to parse <url> as a URL");
  if url.scheme() != "http" {
    eprintln!("only HTTP protocol supported");
    return;
  }
  let domain_name = url.host_str().expect("domain name required");

  let _dns_server: std::net::Ipv4Addr = dns_server_text
      .parse()
      .expect("unable to parse <dns-server> as an IPv4 address");

  let tap = TapInterface::new(&tap_text).expect("unable to use <tap-device> as a network interface");

  let addr = dns::resolve(dns_server_text, domain_name).unwrap().unwrap();

  let mac = ethernet::MacAddress::new().into();

  http::get(tap, mac, addr, url).unwrap();
}