fn main() {
  let x = 5;
  println!("The value of x is: {}", x);
  let x = x + 1;
  println!("The value of x is: {}", x);
  let x = x * 2;

  println!("The value of x is: {}", x);

  let spaces = "   ";
  println!("The value of x is: {}", spaces);
  let spaces = spaces.len();

  println!("The value of x is: {}", spaces);


  let x = "_";
  println!("The value of x is: {}", x);
  let x = x.len();
  println!("The value of x is: {}", x);
  let x = "Привет!";
  println!("The value of x is: {}", x);
  let x: u32 = x.len() as u32;
  println!("The value of x is: {}", x);
  let x = "Привет!";
  println!("The value of x is: {}", x);
  let x: f32 = 3.45;
  println!("The value of x is: {}", x);

  /// cant do this in mut vars
  let mut spacesMut = "   ";
  spacesMut = spacesMut.len();
  println!("The value of x is: {}", spacesMut);
}