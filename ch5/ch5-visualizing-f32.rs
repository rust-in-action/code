const BIAS: i32 = 127;     // <1>
const RADIX: f32 = 2.0;    // <1>

fn main() {                // <2>
  let n: f32 = 42.42;

  let (sign, exp, frac) = to_parts(n);
  let (sign_, exp_, mant) = decode(sign, exp, frac);
  let n_ = from_parts(sign_, exp_, mant);

  println!("{} -> {}", n, n_);
  println!("field    |  as bits | as real number");
  println!("sign     |        {:01b} | {}", sign, sign_);
  println!("exponent | {:08b} | {}", exp, exp_);
  println!("mantissa | {:023b} | {}", frac, mant);
}

fn to_parts(n: f32) -> (u32, u32, u32) {
  let bits = n.to_bits();

  let sign     = (bits >> 31) & 1;    // <3>
  let exponent = (bits >> 23) & 0xff; // <4>
  let fraction =  bits & 0x7fffff ;   // <5>

  (sign, exponent, fraction)          // <6>
}

fn decode(
  sign: u32,
  exponent: u32,
  fraction: u32
) -> (f32, f32, f32) {
  let signed_1 = (-1.0_f32).powf(sign as f32); // <7>

  let exponent = (exponent as i32) - BIAS;     // <8>
  let exponent = RADIX.powf(exponent as f32);  // <8>
  
  let mut mantissa: f32 = 1.0;

  for i in 0..23 {                             // <9>
    let mask = 1 << i;                         // <9>
    let one_at_bit_i = fraction & mask;        // <9>
    if one_at_bit_i != 0 {                     // <9>
      let i_ = i as f32;                       // <9>
      let weight = 2_f32.powf( i_ - 23.0 );    // <9>
      mantissa += weight;                      // <9>
    }                                          // <9>
  }                                            // <9>

  (signed_1, exponent, mantissa)
}

fn from_parts(                                // <10>
  sign: f32,
  exponent: f32,
  mantissa: f32,
) -> f32 {
    sign *  exponent * mantissa
}
