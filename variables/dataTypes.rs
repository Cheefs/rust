fn main() {
 
  //u8
  //u16 
  //u32
  //u64
  //i8
  //i16
  //i32
  //i64
  //isize
  //usize

  let max = <i8>::max_value();
  let mut value = <i8>::min_value();
  loop {
      value = value + 1;

      if value == max {
        println!("value = {}", value);
        break;
      }
  }

  let value = 98_222_000; // "_" - visual divider
  println!("value = {}", value);

  let mut value = 0xff;
  println!("value = {}", value);
  value = 0x_ff;
  println!("value = {}", value);

  let mut value = 0o77;
  println!("value = {}", value);
  value = 0o_77;
  println!("value = {}", value);

  let mut value = 0b1111_0000;
  println!("value = {}", value);
  value = 0b1_111_0000;
  println!("value = {}", value);

  let value = b'A';
  println!("value = {}", value);


  /// compiler set default as i32

  //let value = 9223372036854775807;
  let value:i64 = 9223372036854775807;
  println!("value = {}", value);

  //let value = -9223372036854775808;
  let value:i64 = -9223372036854775808;
  println!("value = {}", value);

  //let value = 18446744073709551615;
  let value:u64 = 18446744073709551615;
  println!("value = {}", value);


  let x = 2.2; // f64
  let y: f32 = 3.3; // f32

  let value32_min = std::f32::MIN;
  println!("value f32 min = {}", value32_min);
  let value32_max = std::f32::MAX;
  println!("value f32 max = {}", value32_max);

  let value64_min = std::f64::MIN;
  println!("value f64 min = {}", value64_min);
  let value64_max = std::f64::MAX;
  println!("value f64 max = {}", value64_max);

//=========== constants f32 ======================

  println!("std::f32");
  // Approximate number of significant digits in base 10.
  println!("DIGITS = {}", std::f32::DIGITS);

  // Difference between 1.0 and the next largest representable number.
  println!("EPSILON = {}", std::f32::EPSILON);

  // Infinity (∞).
  println!("INFINITY = {}", std::f32::INFINITY);

  // Number of significant digits in base 2.
  println!("MANTISSA_DIGITS = {}", std::f32::MANTISSA_DIGITS);

  // Largest finite f32 value.
  println!("MAX = {}", std::f32::MAX);

  // Maximum possible power of 10 exponent.
  println!("MAX_10_EXP = {}", std::f32::MAX_10_EXP);

  // Maximum possible power of 2 exponent.
  println!("MAX_EXP = {}", std::f32::MAX_EXP);

  // Smallest finite f32 value.
  println!("MIN = {}", std::f32::MIN);

  // Minimum possible normal power of 10 exponent.
  println!("MIN_10_EXP = {}", std::f32::MIN_10_EXP);

  // One greater than the minimum possible normal power of 2 exponent.
  println!("MIN_EXP = {}", std::f32::MIN_EXP);

  // Smallest positive normal f32 value.
  println!("MIN_POSITIVE = {}", std::f32::MIN_POSITIVE);

  // Not a Number (NaN).
  println!("NAN = {}", std::f32::NAN);

  // Negative infinity (-∞).
  println!("NEG_INFINITY = {}", std::f32::NEG_INFINITY);

  // The radix or base of the internal representation of f32.
  println!("RADIX = {}", std::f32::RADIX);


  ///======== constnts f64 =================
  
  println!("std::f64");
  // Approximate number of significant digits in base 10.
  println!("DIGITS = {}", std::f64::DIGITS);

  // Difference between 1.0 and the next largest representable number.
  println!("EPSILON = {}", std::f64::EPSILON);

  // Infinity (∞).
  println!("INFINITY = {}", std::f64::INFINITY);

  // Number of significant digits in base 2.
  println!("MANTISSA_DIGITS = {}", std::f64::MANTISSA_DIGITS);

  // Largest finite f64 value.
  println!("MAX = {}", std::f64::MAX);

  // Maximum possible power of 10 exponent.
  println!("MAX_10_EXP = {}", std::f64::MAX_10_EXP);

  // Maximum possible power of 2 exponent.
  println!("MAX_EXP = {}", std::f64::MAX_EXP);

  // Smallest finite f64 value.
  println!("MIN = {}", std::f64::MIN);

  // Minimum possible normal power of 10 exponent.
  println!("MIN_10_EXP = {}", std::f64::MIN_10_EXP);

  // One greater than the minimum possible normal power of 2 exponent.
  println!("MIN_EXP = {}", std::f64::MIN_EXP);

  // Smallest positive normal f64 value.
  println!("MIN_POSITIVE = {}", std::f64::MIN_POSITIVE);

  // Not a Number (NaN).
  println!("NAN = {}", std::f64::NAN);

  // Negative infinity (-∞).
  println!("NEG_INFINITY = {}", std::f64::NEG_INFINITY);

  // The radix or base of the internal representation of f64.
  println!("RADIX = {}", std::f64::RADIX);

}