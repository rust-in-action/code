#![feature(proc_macro_hygiene, decl_macro)] // <1> Enable some compiler features in this crate through the `feature` attribute. The hashbang (`#!`) syntax signifies that the attribute refers whatever the in the current scope of the attribute. In this case, it refers to the whole crate. 

extern crate chrono; // <2> Import an external crate. The  `chrono` create includes time functionality.
#[macro_use]         // <3> The `macro_use` attribute brings macros from an external crate into local scope. The hash (`#`) syntax signifies that the attribute refers to the item that immediately follows the attribute.
extern crate serde_derive; // <4> The `serde_derive` crate can automatically implement JSON serialization for types we define. See `#[derive(Serialize)]` on line 17.
#[macro_use]
extern crate rocket;         // <5> Import the `rocket` crate. Rocket is the web framework used in this example.
extern crate rocket_contrib; // <6> Import the `rocket_contrib` crate. We'll make use of this to generate a HTTP response in a type-safe manner.

use chrono::prelude::*;              // <7> Brings all exported members from `chrono::prelude`, such as `DateTime` and `Utc`, into local scope. The `prelude` submodule makes this glob syntax idiomatic. Many library crates will expose their intended API via a prelude submodule.
use rocket::response::content::Html; // <8> Bring the `Html` type into local scope. This will enable Rocket to create valid HTTP headers, such as Content-Type=text/html.
use rocket_contrib::json::Json;      // <9> Bring `Json` into local scope. `Json` type can automatically create HTTP responses of JSON from types that implement `serde::Serialize`.

#[derive(Serialize)] // <10> Automatically generate a string representation of this struct when required. This capability is provided by `serde_derive` and used by `rocket_contrib::json::Json`.
struct Timestamp {   // <11> Create a custom type `Timestamp`
  t: String,         // <12> Declare the field `t` to be a `String`
}

#[get("/")] // <13> Custom syntax provided by Rocket, via the features enabled on line 1, to connect the HTTP path to this function.
fn index() -> Html<String> {   // <14> Return a `Html` that wraps a `String`. The `Html` type is provided by `rocket::response::content`.
  let content: &str = "
  <h1>Hello, Rust in Action!</h1>
  <p>What is the <a href=\"/now\">time</a>?</p>
  ";
  let content_as_string = String::from(content); // <15> Rust distinguishes between `str` and `String`. `String` has more features, `str` is lightweight.
  Html(content_as_string) // <16> Rust returns the result of the final expression. The "return" keyword is available, but not required.
}

#[get("/now")]
fn now() -> Json<Timestamp> { // <17> Return a `Timestamp` (defined here) within a `Json` (defined in `rocket_contrib::json`). `Json` uses the `Serialize` implementation to convert the `Timestamp` into a valid JSON string.
  let now: DateTime<Utc> = Utc::now(); // <18> Utc::now() is a _static method_
  let timestamp = Timestamp { t: now.to_rfc3339() }; // <19> Rather than constructor functions, Rust uses a literal syntax to create objects.
  Json(timestamp)
}

fn main() {
  rocket::ignite()
      .mount("/", routes![index, now])
      .launch();
}
