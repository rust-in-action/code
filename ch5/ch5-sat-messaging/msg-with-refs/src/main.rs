#[derive(Debug)]
enum StatusCode {
  Ok,
}

#[derive(Debug)]
struct Mailbox {
  messages: Vec<Message>,
}

type Message = String;

#[derive(Debug)]
struct CubeSat {
  id: u64,
  mailbox: Mailbox,
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

fn main() {
    let base = GroundStation {};
    let mut sat_a = CubeSat { id: 0, mailbox: Mailbox { messages: vec![] } };
    let mut sat_b = CubeSat { id: 1, mailbox: Mailbox { messages: vec![] } };
    let mut sat_c = CubeSat { id: 2, mailbox: Mailbox { messages: vec![] } };

    println!("t0: {:?}", sat_a);

    // sat_a.mailbox.messages.push(Message::from("hi"));

    base.send(&mut sat_a, Message::from("hello there!"));

    println!("t1: {:?}", sat_a);

    let msg = sat_a.recv();
    //println!("{:?}", msg);

    println!("t2: {:?}", sat_a);
}
