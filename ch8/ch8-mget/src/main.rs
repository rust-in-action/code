use clap::{App, Arg};
use smoltcp::phy::TapInterface;
use url::Url;

mod dns;
mod ethernet;
mod http;

fn main() {
  let app = App::new("mget")
    .about("GET a webpage, manually")
    .arg(Arg::with_name("url").required(true))       // <1>
    .arg(Arg::with_name("tap-device").required(true))// <2>
    .arg(
      Arg::with_name("dns-server")
        .default_value("1.1.1.1"),   // <3>
    )
    .get_matches();                  // <4>

  let url_text = app.value_of("url").unwrap();
  let dns_server_text =
    app.value_of("dns-server").unwrap();
  let tap_text = app.value_of("tap-device").unwrap();

  let url = Url::parse(url_text)                    // <5>
    .expect("error: unable to parse <url> as a URL");

  if url.scheme() != "http" {                       // <5>
    eprintln!("error: only HTTP protocol supported");
    return;
  }

  let tap = TapInterface::new(&tap_text)            // <5>
    .expect(
      "error: unable to use <tap-device> as a \
       network interface",
    );

  let domain_name =
    url.host_str()                                  // <5>
      .expect("domain name required");

  let _dns_server: std::net::Ipv4Addr =
    dns_server_text
      .parse()                                      // <5>
      .expect(
        "error: unable to parse <dns-server> as an \
         IPv4 address",
      );

  let addr =
    dns::resolve(dns_server_text, domain_name)      // <6>
      .unwrap()
      .unwrap();

  let mac = ethernet::MacAddress::new().into();     // <7>

  http::get(tap, mac, addr, url).unwrap();          // <8>

}
