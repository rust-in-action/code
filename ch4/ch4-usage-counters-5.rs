use std::time;

#[derive(Debug)]
struct UsageCounter {
  count: usize,
  reset_interval: time::Duration,
  last_reset_at: time::SystemTime,
}

impl UsageCounter {
    fn incr(&mut self) {
      self.count += 1;
    }

    fn reset(&mut self) {
      self.count = 0;
      self.last_reset_at = time::SystemTime::now();
    }

    fn check_usage(&mut self) -> (time::SystemTime, usize) {
      let since = self.last_reset_at;
      let usage = self.count;
      let now = time::SystemTime::now();

      let time_for_next_reset = since + self.reset_interval;
      if time_for_next_reset < now {
        self.reset();
      }

      (since, usage)
    }
}

fn main() {
  let start = time::SystemTime::now();
  let hourly = time::Duration::from_secs(60 * 60);

  let mut usage_counters = [
    UsageCounter { count:0, reset_interval: hourly, last_reset_at: start},
    UsageCounter { count:0, reset_interval: hourly, last_reset_at: start},
    UsageCounter { count:0, reset_interval: hourly, last_reset_at: start},
  ];

  usage_counters = run(usage_counters);

  // for uc in &mut usage_counters {
  //   let (reset_at, count) = uc.check_usage();
  //   println!{"{:?} {}", reset_at, count};
  // }

  let (uc_0_count, uc_1_count, uc_2_count) = check_all_usage(&usage_counters);

  println!("{} = {}", usage_counters[0].count, uc_0_count);
  println!("{} = {}", usage_counters[1].count, uc_1_count);
  println!("{} = {}", usage_counters[2].count, uc_2_count);
}

fn check_all_usage(uc_slice: &[UsageCounter; 3]) -> (usize,usize,usize) {
    let uc_address = uc_slice as *const UsageCounter as usize;

    let size_of_uc = std::mem::size_of::<UsageCounter>();

    let uc_0_addr = uc_address;
    let uc_1_addr = uc_0_addr + size_of_uc;
    let uc_2_addr = uc_1_addr + size_of_uc;

    let uc_0_dot_count = unsafe {
        *(uc_0_addr as *const usize)
    };
    let uc_1_dot_count = unsafe {
        *(uc_1_addr as *const usize)
    };
    let uc_2_dot_count = unsafe {
        *(uc_2_addr as *const usize)
    };

    (uc_0_dot_count, uc_1_dot_count, uc_2_dot_count)
}

fn run(mut usage_counters: [UsageCounter; 3]) -> [UsageCounter; 3] {
  for _ in 0..50 {
    for uc in &mut usage_counters {
      uc.incr();
    }
  }

  usage_counters[1].reset();

  usage_counters
}
