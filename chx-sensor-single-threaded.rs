use std::time;
use std::f64::consts::PI;
use std::thread;

fn main() {
    let mut state: f64 = 1.0;

    for i in 1.. {
        let delay_ms = if i % 4 == 0 {
            500
        } else if i % 7 == 0 {
            1000
        } else {
            50
        };

        let delay = time::Duration::from_millis(delay_ms);
        thread::sleep(delay);

        let reading = take_reading(state);
        println!("{}", reading);
        state += 0.05;
    }
}

fn take_reading(x: f64) -> f64 {
  // sin(x) + sin(π / sin(πx))
  x.sin()   + (PI / (x*PI).sin()).sin()
}
