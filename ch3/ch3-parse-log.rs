#[derive(Debug)] // <1> Enable this enum to be printed to the screen via auto-generated code 
enum Event {
    Update,  // <2> Create three variants of Event, including one value for unrecognized events
    Delete,  // <2>
    Unknown, // <2>
}

type Message = String; // <3> A convenient name for String for use in this crate's context 

fn parse_log(line: &'static str) -> (Event, Message) { // <4> A function for parsing a line and converting it into semi-structured data 
  let parts: Vec<&str> = line.splitn(2, ' ').collect(); // <5> `collect()` consumes an iterator (returned from `line.splitn()`) and returns `Vec<T>`
  if parts.len() == 1 {  // <6> If `line.splitn()` didn't split `log` into two parts, return an error 
    return (Event::Unknown, String::from(line))
  }

  let event = parts[0];              // <7> Assign each part to a variable for ease of future use
  let rest = String::from(parts[1]); // <7>

  match event {  
    "UPDATE" | "update" => (Event::Update, rest), // <8> When we match a known event, return structured data
    "DELETE" | "delete" => (Event::Delete, rest), // <8>
    _ => (Event::Unknown, String::from(line)), // <9> If we don't recognize the event type, return the whole line 
  }
}

fn main() {
  let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

  for line in log.lines(){
    let parse_result = parse_log(line);
    println!("{:?}", parse_result);
  }
}