use std::io;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time;
use std::thread;

const DELAY: u64 = 5000;

fn reader_thread(tx: Sender<f64>) {
    loop {
        //let reading: f64;
        let mut input = String::new();

        let read_result = io::stdin().read_line(&mut input);

        match read_result {
            Ok(_n_bytes) => {
                // nothing to do
            }
            Err(err) => {
                println!("error reading from stdin: {:#?}", err);
                return;
            }
        }

        let parse_result = input.trim().parse::<f64>();

        if parse_result.is_ok() {
            let reading = parse_result.unwrap();
            tx.send(reading);
//            println!("{}", reading);
        } else {
            println!("error parsing line as number: {:#?}", input);
            continue;
        }

        //break;
    }
}

fn writer_thread(rx: Receiver<f64>) {
    let delay = time::Duration::from_millis(DELAY);

    for reading in rx.iter() {
        let received_at = time::Instant::now(); // well, close enough

        println!("-> {} (recv_at:{:?}) ", reading, received_at);

        thread::sleep(delay);
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let reader = thread::spawn(move || reader_thread(tx) );
    let writer = thread::spawn(|| writer_thread(rx) );
    
    reader.join();
    writer.join();
}
