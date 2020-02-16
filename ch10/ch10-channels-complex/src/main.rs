#[macro_use]
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck { // <1> Defining a bespoke message type makes interpreting messages later very simple.
  Ping,                  // <1>
  Pong,                  // <1>
  Pang,                  // <1>
}                        // <1>

fn main() {
  let n_messages = 3;
  let (requests_tx, requests_rx) = unbounded();
  let (responses_tx, responses_rx) = unbounded();

  thread::spawn(move || loop {
    match requests_rx.recv().unwrap() {
      Pong => eprintln!("unexpected pong response"),
      Ping => responses_tx.send(Pong).unwrap(),
      Pang => return, // <2>
    }
  });

  for _ in 0..n_messages {
    requests_tx.send(Ping).unwrap();
  }
  requests_tx.send(Pang).unwrap();

  for _ in 0..n_messages {
    select! {
       recv(responses_rx) -> msg => println!("{:?}", msg),
    }
  }
}
