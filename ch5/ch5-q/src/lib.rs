/// Q1_7 - single byte representation of a fixed point number with range [-1, 1]. 
/// The name refers to the Texas Instrument representation
/// 
/// References:
///  - English Wikipedia: "Q (number format)" https://en.wikipedia.org/wiki/Q_(number_format)
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Q7(i8); // tuple struct holding a i8 value

impl From<f64> for Q7 {
    fn from (n: f64) -> Self {
        // assert!(n >= -1.0);
        // assert!(n <= 1.0);
        if n >= 1.0 { // out of bounds numbers are coerced to the maximum of the range
            Q7(127)
        } else if n <= -1.0 {
            Q7(-128)
        } else {
            Q7((n * 128.0) as i8) // 128 == (2 ** 7) ==  pow(2,7)
        }
    }
}

impl From<Q7> for f64 {
    fn from(n: Q7) -> f64 {
        (n.0 as f64) * 2f64.powf(-7.0) // 0.0078125// (2 ** -7) // pow(2, -7)
    }
}

impl From<f32> for Q7 {
    fn from (n: f32) -> Self {
        Q7::from(n as f64) // conversion from f32 to f64 works perfectly
    }
}

impl From<Q7> for f32 {
    fn from(n: Q7) -> f32 {
        f64::from(n) as f32 // conversion from f64 to f32 can result in undefined behavior, 
                            // but not here as f32 can represent all values representable by Q7
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn out_of_bounds() {
        assert_eq!(Q7::from(10.), Q7::from(1.));
        assert_eq!(Q7::from(-10.), Q7::from(-1.));
    }

    #[test]
    fn f32_to_q7() {
        let n1: f32 = 0.7;
        let q1 = Q7::from(n1);

        let n2 = -0.4;
        let q2 = Q7::from(n2);

        let n3 = 123.0;
        let q3 = Q7::from(n3);

        assert_eq!(q1, Q7(89));
        assert_eq!(q2, Q7(-51));
        assert_eq!(q3, Q7(127));
    }

    #[test]
    fn q7_to_f32() {
        let q1 = Q7::from(0.7);
        let n1 = f32::from(q1);
        assert_eq!(n1, 0.6953125);

        let q2 = Q7::from(n1);   // numbers that can be represented exactly by Q7
        let n2 = f32::from(q2);  // can survive the transition between Q7 and f32
        assert_eq!(n1, n2);
    }
}
