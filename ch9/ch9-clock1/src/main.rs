extern crate chrono;
extern crate clap;

use clap::{App,Arg};
use chrono::{DateTime}; // date type
use chrono::{Local}; // timezone types

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
        .about("Gets and sets (aspirationally) the time.")
        .arg(Arg::with_name("action")
            .takes_value(true)
            .possible_values(&["get", "set"])
            .default_value("get"))
        .arg(Arg::with_name("std")
            .short("s")
            .long("use-standard")
            .takes_value(true)
            .possible_values(&["rfc2822", "rfc3339", "timestamp"])
            .default_value("rfc3339"))
        .arg(Arg::with_name("datetime")
            .help("When <action> is 'set', apply <datetime>. Otherwise, ignore."));

    let args = app.get_matches();

    let action = args.value_of("std").unwrap(); // default_value() has been supplied,
    let std = args.value_of("std").unwrap();    // so it's safe to use .unwrap()

    if action == "set" {
        unimplemented!() // break early
    }

    let now = Clock::get();
    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822"   => println!("{}", now.to_rfc2822()),
        "rfc3339"   => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
