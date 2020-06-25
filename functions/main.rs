fn main() {
  println!("Hello, world!");

  // another_function();
  // another_function_with_arguments(4, 6);

  let x = 5;  // operator
  println!("The value of x is: {}", x);
  let y = {  // statement returns 4 to y variable
      let x = 3;
      x + 1 // not end with ";" if ends  -> this statement become operator
  };

  println!("The value of y is: {:?}", y);


  println!("The value of five is: {}", five() );

  // println!("The value of five is: {}", plus_one( 3 ) );
}

fn another_function() {
  println!("Another function.");
}

fn another_function_with_arguments(x: i32, y: i32) {
  println!("The value of x is: {}", x);
  println!("The value of y is: {}", y);
}

/// this will return 5 
fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1;  // compile error
}