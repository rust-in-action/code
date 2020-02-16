use std::time;
use std::thread;
use std::f64::consts::PI;


fn main() {
    // perform work in a child thread so that
    // the process can still be responsive on Ctrl+C etc
    let child = std::thread::spawn(run);
    child.join();
}

struct Sensor {
    state: f64,
    step: f64,
}

impl Sensor {
    fn take_reading(&mut self) -> f64 {
        *self.state += self.step;
        let x = self.state;

        x.sin() + (PI / (x*PI).sin()).sin()
    }
}

fn run() {
    let sensor = Sensor {
        state: 1.0,
        step: 0.25,
    };

    for i in 1.. {
        let delay_ = if i % 5 == 0 {
            500
        } else if i % 3 == 0 {
            1000
        } else {
            50
        };

        let delay = time::Duration::from_millis(delay_);
        thread::sleep(delay);

        let reading = sensor.take_reading();
        println!("{}: f({}) -> {} ({})", i, sensor.state, reading, delay_);
    }
}

fn take_reading(x: f64) -> f64 {
    // sin(x) + sin(π / sin(πx))

}
