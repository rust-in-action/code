extern crate chrono;
extern crate chrono_tz;

use chrono::prelude::*;
use chrono::{Duration};
use chrono_tz::America::Sao_Paulo;

fn main() {
    let t = Utc.timestamp(1431648000, 0);
    assert_eq!(t.to_string(), "2015-05-15 00:00:00 UTC");
    println!("{:?}", t);


    let sp1 = Sao_Paulo.ymd(1996, 2, 11).and_hms(23, 59, 59);
    let sp2 = Sao_Paulo.ymd(1996, 2, 11).and_hms(23, 59, 59) + Duration::seconds(1);
    println!("{} {}", sp1.to_string(), sp1.timestamp());
    println!("{} {}", sp2.to_string(), sp2.timestamp(), );
}