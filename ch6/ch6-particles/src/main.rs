use graphics::math::{Vec2d, add, mul_scalar};    // <1>

use piston_window::*;                            // <2>

use rand::prelude::*;                            // <3>

use std::alloc::{GlobalAlloc, System, Layout};   // <4>

use std::time::Instant;                          // <5>


#[global_allocator]                              // <6>
static ALLOCATOR: ReportingAllocator = ReportingAllocator;

struct ReportingAllocator;                       // <7>

unsafe impl GlobalAlloc for ReportingAllocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let start = Instant::now();
    let ptr = System.alloc(layout);              // <8>
    let end = Instant::now();
    let time_taken = end - start;
    let bytes_requested = layout.size();

    eprintln!("{}\t{}", bytes_requested, time_taken.as_nanos());
    ptr
  }

  unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
    System.dealloc(ptr, layout);
  }
}

struct World {                      // <9>
  current_turn: u64,                // <9>
  particles: Vec<Box<Particle>>,    // <9>
  height: f64,                      // <9>
  width: f64,                       // <9>
  rng: ThreadRng,                   // <9>
}

struct Particle {                   // <10>
  height: f64,                      // <10>
  width: f64,                       // <10>
  position: Vec2d<f64>,             // <10>
  velocity: Vec2d<f64>,             // <10>
  acceleration: Vec2d<f64>,         // <10>
  color: [f32; 4],                  // <10>
}

impl Particle {
  fn new(world : &World) -> Particle {
    let mut rng = thread_rng();
    let x = rng.gen_range(0.0..=world.width);      // <11>
    let y = world.height;                          // <11>
    let x_velocity = 0.0;                          // <12>
    let y_velocity = rng.gen_range(-2.0..0.0);     // <12>
    let x_acceleration = 0.0;                      // <13>
    let y_acceleration = rng.gen_range(0.0..0.15); // <13>

    Particle {
      height: 4.0,
      width: 4.0,
      position: [x, y].into(),                     // <14>
      velocity: [x_velocity, y_velocity].into(),   // <14>
      acceleration: [x_acceleration,
	                 y_acceleration].into(),       // <14>
      color: [1.0, 1.0, 1.0, 0.99],                // <15>
    }
  }

  fn update(&mut self) {
    self.velocity = add(self.velocity,
	                    self.acceleration);        // <16>
    self.position = add(self.position,
	                    self.velocity);            // <16>
    self.acceleration = mul_scalar(                // <17>
      self.acceleration,                           // <17>
      0.7                                          // <17>
    );                                             // <17>
    self.color[3] *= 0.995;                        // <18>
  }
}

impl World {
  fn new(width: f64, height: f64) -> World {
    World {
      current_turn: 0,
      particles: Vec::<Box<Particle>>::new(),      // <19>
      height: height,
      width: width,
      rng: thread_rng(),
    }
  }

  fn add_shapes(&mut self, n: i32) {
    for _ in 0..n.abs() {
      let particle = Particle::new(&self);         // <20>
      let boxed_particle = Box::new(particle);     // <21>
      self.particles.push(boxed_particle);         // <22>
    }
  }

  fn remove_shapes(&mut self, n: i32) {
    for _ in 0..n.abs() {
      let mut to_delete = None;

      let particle_iter = self.particles           // <23>
        .iter()                                    // <23>
        .enumerate();                              // <23>

      for (i, particle) in particle_iter {         // <24>
        if particle.color[3] < 0.02 {              // <24>
          to_delete = Some(i);                     // <24>
        }                                          // <24>
        break;                                     // <24>
      }                                            // <24>
                                                   // <24>
      if let Some(i) = to_delete {                 // <24>
        self.particles.remove(i);                  // <24>
      } else {                                     // <24>
        self.particles.remove(0);                  // <24>
      };                                           // <24>
    }
  }

  fn update(&mut self) {
    let n = self.rng.gen_range(-3..=3);            // <25>

    if n > 0 {
      self.add_shapes(n);
    } else {
      self.remove_shapes(n);
    }

    self.particles.shrink_to_fit();
    for shape in &mut self.particles {
      shape.update();
    }
    self.current_turn += 1;
  }
}

fn main() {
  let (width, height) = (1280.0, 960.0);
  let mut window: PistonWindow = WindowSettings::new(
    "particles", [width, height]
  )
  .exit_on_esc(true)
  .build()
  .expect("Could not create a window.");

  let mut world = World::new(width, height);
  world.add_shapes(1000);

  while let Some(event) = window.next() {
    world.update();

    window.draw_2d(&event, |ctx, renderer, _device| {
      clear([0.15, 0.17, 0.17, 0.9], renderer);

      for s in &mut world.particles {
        let size = [s.position[0], s.position[1], s.width, s.height];
        rectangle(s.color, size, ctx.transform, renderer);
      }
    });
  }
}
