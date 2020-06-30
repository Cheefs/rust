pub fn declare() {
  println!("============== /declare ===================");
  let vec1: Vec<i32> = Vec::new();
  let vec2 = vec![1, 2, 3];

  let vec3 = vec![1, 2, (3 as u64)];

  println!("vec1:  {:?}, vec2:  {:?}, vec3:  {:?}", vec1, vec2 , vec3 );
  println!("============== /declare ===================");
}

pub fn fill() {
  println!("============== fill ===================");
  let mut vec = vec![];
  vec.push(0);
  vec.push(2);
  vec.push(10);

  println!( "fill_vector:  {:?}", vec );

  println!("============== /fill ===================");
}

pub fn get_value() {
  println!("============== get_value ===================");
  let v = vec![1, 2, 3, 4, 5];

  let third: &i32 = &v[2];
  println!("&v[2]:  {}", third);
  let third: Option<&i32> = v.get(2); // will return Option<&T>
  println!("v.get(2):  {:?}", third);

  let v = vec![1, 2, 3, 4, 5];

  // let does_not_exist = &v[100];  /// or println!("{}", v[100]); take error on compile
  // println!("does_not_exist &v[100] {:?}", does_not_exist);
  let does_not_exist = v.get(100);
  println!("does_not_exist v.get(100): {:?}", does_not_exist);  // will return None
  println!("============== /get_value ===================");
}

/** @link https://doc.rust-lang.org/stable/nomicon/vec.html. */
pub fn lost_control_error () {
  let mut v = vec![1, 2, 3, 4, 5];
  //  v.push(6);  // to resolve error ned put here
  let first = &v[0];
  v.push(6);

  // println!("{:?}", first ); // error 
}

pub fn with_enums() {
  println!("============== with_enums ===================");
  #[derive(Debug)]
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
  println!("row: {:?}", row);
  println!("============== /with_enums ===================");
}