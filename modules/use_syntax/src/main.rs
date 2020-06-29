pub mod a {
  pub mod series {
      pub mod of {
          pub fn nested_modules() {
              println!("nested_modules");
          }
      }
  }
}

use a::series::of;
// use a::series::of::nested_modules;  <- static import


// enums has own namespace
#[derive(Debug)]
enum TrafficLight {
  Red,
  Yellow,
  Green,
}
use TrafficLight::{ Red, Yellow };
// use TrafficLight::*; <- "glob" import ALL visiable elements from namespase Red, Yellow, Green

fn main() {
  of::nested_modules();

  let red = Red;
  let yellow = Yellow;
  let green = TrafficLight::Green;

  println!("{:?}",red);
  println!("{:?}",yellow);
  println!("{:?}",green);
}

/*
  // conflict case axample

  pub mod a {
    pub mod series {
        pub mod of1 {
            pub fn nested_modules() {
                println!("nested_modules 1");
            }
        }
        pub mod of2 {
            pub fn nested_modules() {
                println!("nested_modules 2");
            }
        }
    }
  }
  use a::series::of1::*;
  use a::series::of2::*;

  fn main() {
    nested_modules();
  }
*/
