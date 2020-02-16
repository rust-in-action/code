#![feature(plugin)] // <1>
#![plugin(rocket_codegen)] // <1>

extern crate serde;          // <2>
extern crate chrono;         // <2>
extern crate rocket;         // <2>
extern crate rocket_contrib; // <2>

#[macro_use]  // <3> Syntax to indicate that we want to import macros from another module
extern crate serde_derive;   // <3>

use chrono::prelude::*; // <4> brings all exported members into local scope (e.g. DateTime and Utc)
use rocket_contrib::{Json}; // <5> bring single member into local scope

#[derive(Serialize)] // <6> Automatically generate a string representation of this struct (which will be used as JSON)
struct Timestamp { // <7> Syntax to create a custom type 
    time: String,  // <8> The `Timestamp` `time` field is of type `String`
}

#[get("/")] // <9> Custom syntax provided by the library that indicates to  code generation
fn index() -> &'static str { // <10> Define a function with no arguments and its return type
    "Hello, world!" // <11> Rust returns the result of the final expression 
}

#[get("/time")]
fn time_now() -> Json<Timestamp> {
    let now: DateTime<Utc> = Utc::now();
    let timestamp = Timestamp { time: now.to_rfc3339() };
    Json(timestamp)
}

fn main() {
    rocket::ignite()
            .mount("/", routes![index, time_now])
            .launch();
}