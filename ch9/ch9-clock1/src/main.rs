use chrono::DateTime;
use chrono::Local;
use clap::{App, Arg};

struct Clock;

impl Clock {
  fn get() -> DateTime<Local> {
    Local::now()
  }

  fn set() -> ! {
    unimplemented!()
  }
}

fn main() {
  let app = App::new("clock")
    .version("0.1")
    .about("Gets and (aspirationally) sets the time.")
    .arg(
      Arg::with_name("action")
        .takes_value(true)
        .possible_values(&["get", "set"])
        .default_value("get"),
    )
    .arg(
      Arg::with_name("std")
        .short("s")
        .long("use-standard")
        .takes_value(true)
        .possible_values(&[
          "rfc2822",
          "rfc3339",
          "timestamp",
        ])
        .default_value("rfc3339"),
    )
    .arg(Arg::with_name("datetime").help(
      "When <action> is 'set', apply <datetime>. \
       Otherwise, ignore.",
    ));

  let args = app.get_matches();

  let action = args.value_of("action").unwrap();   // <1>
  let std = args.value_of("std").unwrap();         // <1>

  if action == "set" {
    unimplemented!()                               // <2>
  }

  let now = Clock::get();
  match std {
    "timestamp" => println!("{}", now.timestamp()),
    "rfc2822" => println!("{}", now.to_rfc2822()),
    "rfc3339" => println!("{}", now.to_rfc3339()),
    _ => unreachable!(),
  }
}
