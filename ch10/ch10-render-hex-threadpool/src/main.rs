extern crate crossbeam;
use std::thread;
use std::env;

use crossbeam::channel::{unbounded};
use svg::Document;
use svg::node::element::{Path, Rectangle};
use svg::node::element::path::{Command, Position, Data};

use crate::Operation::{Forward, TurnLeft, TurnRight, Home, Noop};
use crate::Orientation::{North, East, West, South};

const WIDTH: isize = 400;
const HEIGHT: isize = WIDTH;
const HOME_Y: isize = HEIGHT/2;
const HOME_X: isize = WIDTH/2;
const STROKE_WIDTH: usize = 5;

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North, East, West, South,
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Forward(isize),
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8),
}

#[derive(Debug)]
struct Artist {
    x: isize,
    y: isize,
    heading: Orientation,
}

impl Artist {
    fn new() -> Artist {
        Artist {
            heading: North,
            x: HOME_X,
            y: HOME_Y,
        }
    }
    fn home(&mut self) {
        self.x = HOME_X;
        self.y = HOME_Y;
    }

    fn forward(&mut self, distance: isize) {
        match self.heading {
            North => self.y += distance,
            South => self.y -= distance,
            West  => self.x += distance,
            East  => self.x -= distance,
        }
    }

    fn turn_right(&mut self) {
        self.heading = match self.heading {
            North => East,
            South => West,
            West  => North,
            East  => South,
        }
    }

    fn turn_left(&mut self) {
        self.heading = match self.heading {
            North => West,
            South => East,
            West  => South,
            East  => North,
        }
    }

    fn wrap(&mut self) {
        if self.x < 0 {
            self.x = HOME_X;
            self.heading = West;
        } else if self.x > WIDTH {
            self.x = HOME_X;
            self.heading = East;
        }

        if self.y < 0 {
            self.y = HOME_Y;
            self.heading = North;
        } else if self.y > HEIGHT {
            self.y = HOME_Y;
            self.heading = South;
        }
    }
}

enum Work { // <1> Create a type for the messages that we will send through the channels.
Task((usize, u8)), // <2> The `usize` field of this tuple will indicate the position that the byte has that's being processed. This is necessary because they can be returned out of order.
Finished, // <3> We'll give worker threads a marker message to indicate that it's time for them to shut themselves down.
}

fn parse_byte(byte: u8) -> Operation { // <4> Extract the functionality that workers will need to carry out to simplify the logic.
    match byte {
        b'0' => Home,
        b'1'..=b'9' => {
            let distance = (byte - 0x30) as isize;
            Forward(distance * (HEIGHT/10))
        },
        b'a' | b'b' | b'c' => TurnLeft,
        b'd' | b'e' | b'f' => TurnRight,
        _ => Noop(byte),
    }
}

fn parse(input: &str) -> Vec<Operation> {
    let n_threads = 2;
    let (todo_tx, todo_rx) = unbounded(); // <5> Create one channel for tasks to be completed.
    let (results_tx, results_rx) = unbounded(); // <6> Create one channel for the decoded instructions to be returned to.
    let mut n_bytes = 0;
    for (i,byte) in input.bytes().enumerate() {
        todo_tx.send(Work::Task((i,byte))).unwrap(); // <7> Fill the task queue with work.
        n_bytes += 1; // <8> Keep track of how many tasks there are to do
    }

    for _ in 0..n_threads {                     // <9> Send each thread the signal that it's time to shut down.
        todo_tx.send(Work::Finished).unwrap();  //
    }                                           //

    for _ in 0..n_threads {
        let todo = todo_rx.clone();             // <10> When cloned, channels can be shared between threads.
        let results = results_tx.clone();       //
        thread::spawn(move || {
            loop {
                let task = todo.recv();
                let result = match task {
                    Err(_) => break,
                    Ok(Work::Finished) => break,
                    Ok(Work::Task((i, byte))) => (i, parse_byte(byte)),
                };
                results.send(result).unwrap();

            }
        });
    }
    let mut ops = vec![Noop(0); n_bytes];  // <11> Because results can be returned in arbitrary order, we'll initalize a complete `Vec<Command>` that will be overwritten by our incoming results. A vector is used, rather than an array because that is what is used by the type signature and we don't want to refactor the whole program to suit this new implementation.
    for _ in 0..n_bytes {
        let (i, op) = results_rx.recv().unwrap();
        ops[i] = op;
    }
    ops
}

fn convert(operations: &Vec<Operation>) -> Vec<Command> {
    let mut turtle = Artist::new();

    let mut path_data = Vec::<Command>::with_capacity(1+operations.len());
    path_data.push(Command::Move(Position::Absolute, (HOME_X, HOME_Y).into()));

    for op in operations {
        match *op {
            Forward(distance) => turtle.forward(distance),
            TurnLeft          => turtle.turn_left(),
            TurnRight         => turtle.turn_right(),
            Home              => turtle.home(),
            Noop(byte)        => eprintln!("warning: illegal byte encountered: {:?}", byte),
        };
        path_data.push(Command::Line(Position::Absolute, (turtle.x, turtle.y).into())); 
        turtle.wrap();
    }
    path_data
}

fn generate_svg(path_data: Vec<Command>) -> Document {
    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff");

    let border = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", WIDTH)
        .set("height", HEIGHT)
        .set("fill", "#ffffff")
        .set("fill-opacity", "0.0")
        .set("stroke", "#cccccc")
        .set("stroke-width", 3*STROKE_WIDTH);

    let sketch = Path::new()
        .set("fill", "none")
        .set("stroke", "#2f2f2f")
        .set("stroke-width", STROKE_WIDTH)
        .set("stroke-opacity", "0.9")
        .set("d", Data::from(path_data));

    let document = Document::new()
        .set("viewBox", (0, 0, HEIGHT, WIDTH))
        .set("height", HEIGHT)
        .set("width", WIDTH)
        .set("style", "style=\"outline: 5px solid #800000;\"" )
        .add(background)
        .add(sketch)
        .add(border);

    document
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let input = args.get(1).unwrap();
    let default_filename = format!("{}.svg", input);
    let save_to = args.get(2).unwrap_or(&default_filename);

    let operations = parse(input);
    let path_data = convert(&operations);
    let document = generate_svg(path_data);
    svg::save(save_to, &document).unwrap();
}
