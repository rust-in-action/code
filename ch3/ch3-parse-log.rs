#[derive(Debug)]         // <1>
enum Event {
    Update,              // <2>
    Delete,              // <2>
    Unknown,             // <2>
}

type Message = String;   // <3>

fn parse_log(line: &str) -> (Event, Message) {   // <4>
  let parts: Vec<_> = line                       // <5>
                      .splitn(2, ' ')
                      .collect();                // <6>
  if parts.len() == 1 {                          // <7>
    return (Event::Unknown, String::from(line))
  }

  let event = parts[0];                // <8>
  let rest = String::from(parts[1]);   // <8>

  match event {
    "UPDATE" | "update" => (Event::Update, rest),  // <9>
    "DELETE" | "delete" => (Event::Delete, rest),  // <9>
    _ => (Event::Unknown, String::from(line)),    // <10>
  }
}

fn main() {
  let log = "BEGIN Transaction XK342
UPDATE 234:LS/32231 {\"price\": 31.00} -> {\"price\": 40.00}
DELETE 342:LO/22111";

  for line in log.lines() {
    let parse_result = parse_log(line);
    println!("{:?}", parse_result);
  }
}
