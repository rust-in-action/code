extern crate chrono;

use chrono::{Local}; // timezone type

fn main() {    
    let now = Local::now();
    println!("{}", now);
}
