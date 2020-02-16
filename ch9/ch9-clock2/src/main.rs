extern crate chrono;
extern crate clap;
#[cfg(windows)]
extern crate kernel32;
#[cfg(not(windows))]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

use chrono::DateTime; // date type
use chrono::Weekday;
use chrono::{Datelike, Timelike}; // traits that provides .year(), .month(), .second() ... methods
use chrono::{FixedOffset, Local, NaiveDateTime, Utc}; // timezone types
use clap::{App, Arg};
use std::mem::zeroed;
use std::thread::sleep;
struct Clock;

impl Clock {
    fn get() -> DateTime<Local> {
        Local::now()
    }

    #[cfg(windows)]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use chrono::Weekday;
        use kernel32::SetSystemTime;
        use winapi::{SYSTEMTIME, WORD};

        let t = t.with_timezone(&Local);

        let mut systime: SYSTEMTIME = unsafe { zeroed() };

        let dow = match t.weekday() {
            Weekday::Mon => 1,
            Weekday::Tue => 2,
            Weekday::Wed => 3,
            Weekday::Thu => 4,
            Weekday::Fri => 5,
            Weekday::Sat => 6,
            Weekday::Sun => 0,
        };

        let mut ns = t.nanosecond();
        let is_leap_second = ns > 1_000_000_000;

        if is_leap_second {
            ns -= 1_000_000_000;
        }

        systime.wYear = t.year() as WORD;
        systime.wMonth = t.month() as WORD;
        systime.wDayOfWeek = dow as WORD;
        systime.wDay = t.day() as WORD;
        systime.wHour = t.hour() as WORD;
        systime.wMinute = t.minute() as WORD;
        systime.wSecond = t.second() as WORD;
        systime.wMilliseconds = (ns / 1_000_000) as WORD;

        let systime_ptr = &systime as *const SYSTEMTIME; // as *mut SYSTEMTIME; // convert to a pointer, then change its mutability

        unsafe {
            SetSystemTime(systime_ptr); // giving a pointer to a value to something outside of Rust's control is unsafe
        }
    }

    #[cfg(not(windows))]
    fn set<Tz: TimeZone>(t: DateTime<Tz>) -> () {
        use libc::settimeofday;
        use libc::{suseconds_t, time_t, timeval, timezone};

        let t = t.with_timezone(&Local); // variable names indicate the data's progression
        let mut u: timeval = unsafe { zeroed() }; // through the function

        u.tv_sec = t.timestamp() as time_t;
        u.tv_usec = t.timestamp_subsec_micros() as suseconds_t;

        unsafe {
            let mock_tz: *const timezone = std::ptr::null();
            settimeofday(&u as *const timeval, mock_tz);
        }
    }
}

fn main() {
    let app = App::new("clock")
        .version("0.1.2")
        .about("Gets and (aspirationally) sets the time.")
        .after_help("Note: UNIX timestamps are parsed as whole seconds since 1st January 1970 0:00:00 UTC. For more accuracy, use another format.")
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
    let std = args.value_of("std").unwrap(); // so it's safe to use .unwrap()

    if action == "set" {
        let t_ = args.value_of("datetime").unwrap();

        let parser = match std {
            "rfc2822" => DateTime::parse_from_rfc2822,
            "rfc3339" => DateTime::parse_from_rfc3339,
            //"rfc3339" => |secs:| NaiveDateTime,
            _ => unimplemented!(),
        };

        let err_msg = format!("Unable to parse {} according to {}", t_, std);
        let t = parser(t_).expect(&err_msg);

        Clock::set(t)
    }

    let now = Clock::get();

    match std {
        "timestamp" => println!("{}", now.timestamp()),
        "rfc2822" => println!("{}", now.to_rfc2822()),
        "rfc3339" => println!("{}", now.to_rfc3339()),
        _ => unreachable!(),
    }
}
