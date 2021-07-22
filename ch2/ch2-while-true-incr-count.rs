use std::time::{Duration, Instant};                //<1>

fn main() {
   let mut count = 0;
   let time_limit = Duration::new(1,0);            //<2>
   let start = Instant::now();                     //<3>

   while (Instant::now() - start) < time_limit {   //<4>
       count += 1;
   }
   println!("{}", count);
}