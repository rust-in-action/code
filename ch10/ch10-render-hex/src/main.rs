use std::env;

use svg::Document;
use svg::node::element::{Path, Rectangle};
use svg::node::element::path::{Command, Position, Data};

use crate::Operation::{Forward, TurnLeft, TurnRight, Home, Noop}; // <1> `Operation` and `Orientation` enum types are defined later in the file.
use crate::Orientation::{North, East, West, South};                 // <1> Including the `use` here removes a lot of noise from the source code later on.

const WIDTH: isize = 400;     // <1> `HEIGHT` and `WIDTH` provide the bounds of the drawing. 
const HEIGHT: isize = WIDTH;  // <1> 
const HOME_Y: isize = HEIGHT/2; // <2> `HOME_Y` and `HOME_X` constants allow us to easily reset where we are drawing from to the middle. _y_ is the vertical coordinate, _x_ is the horizontal.
const HOME_X: isize = WIDTH/2;  // <2> 
const STROKE_WIDTH: usize = 5; // <3> `STROKE_WIDTH` is a parameter for the output SVG. 

#[derive(Debug, Clone, Copy)]
enum Orientation {
    North, East, West, South, // <4> Using descriptions, rather than numerical values, avoids mathematics.
}

#[derive(Debug, Clone, Copy)]
enum Operation {    // <5> To produce richer output, feel free to extend the operations available to your programs. 
    Forward(isize), // <6> Using an `isize` allows you to extend this example to implement a "Reverse" operation without adding a new variant.
    TurnLeft,
    TurnRight,
    Home,
    Noop(u8), // <7> Noop is used when we encounter illegal input. To write error messages, we retain the illegal byte.
}

#[derive(Debug)]
struct Artist { // <8> The `Artist` struct maintains the current state.
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

    fn forward(&mut self, distance: isize) { // <9>
        match self.heading {                 // <9>
            North => self.y += distance,     // <9> `forward()` mutates `self` within the match expression. 
            South => self.y -= distance,     // <9>
            West  => self.x += distance,     // <9>
            East  => self.x -= distance,     // <9>
        }                                    // <9>
    }                                        // <9>

    fn turn_right(&mut self) {              // <10>
        self.heading = match self.heading { // <10> `turn_left()` and `turn_right()` mutate `self` outside of the match expression.
            North => East,                  // <10>
            South => West,                  // <10>
            West  => North,                 // <10>
            East  => South,                 // <10>
        }                                   // <10>
    }                                       // <10>
                                            // <10>
    fn turn_left(&mut self) {               // <10>
        self.heading = match self.heading { // <10>
            North => West,                  // <10>
            South => East,                  // <10>
            West  => South,                 // <10>
            East  => North,                 // <10>
        }                                   // <10>
    }                                       // <10>

    fn wrap(&mut self) {  // <11> `wrap()` ensures that the drawing stays within bounds.
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

fn parse(input: &str) -> Vec<Operation> {
    let mut steps = Vec::<Operation>::new();
    for byte in input.bytes() {
        let step = match byte {
            b'0' => Home,
            b'1'..=b'9' => {
                let distance = (byte - 0x30) as isize; // In ASCII, numerals start at at 0x30 (48). So this converts the `u8` value of `b'2'` to 2. Performing this operation on the whole range of `u8` could cause a panic. We are safe here, thanks to guarantee provided by our pattern matching.
                Forward(distance * (HEIGHT/10))
            },
            b'a' | b'b' | b'c' => TurnLeft,
            b'd' | b'e' | b'f' => TurnRight,
            _ => Noop(byte), // Although we don't expect any illegal characters, there may be some in the input stream. Using a Noop operation allows us to decouple parsing from producing output.
        };
        steps.push(step);
    }
    steps
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

    let border = background.clone()
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
