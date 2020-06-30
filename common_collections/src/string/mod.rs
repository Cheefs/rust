// all strings coded at UTF-8
pub fn declare() {
  println!("============== declare ===================");
  let s = String::new();
  println!("s {}", s );

  // let data = "initial contents";
  // let s = data.to_string();
  // let s = String::from("initial contents"); 
  // the method also works on a literal directly:
  // let s = "initial contents".to_string();

  println!("============== /declare ===================");
}

pub fn concat() {
  println!("============== concat ===================");

  let mut s = String::from("foo");
  s.push_str("bar");
  println!("{}", s);

  let mut s1 = String::from("foo");
  let s2 = String::from("bar");
  s1.push_str(&s2);
  println!("{}", s1);
  println!("{}", s2);
  println!("{}", s2);

  let mut s = String::from("lo");
  s.push('l'); 

  println!("{}", s);

  let s1 = String::from("Hello, ");
  let s2 = String::from("world!");
  let s3 = s1 + &s2; // Note that s1 has been moved here and can no longer be used
  println!("{}", s3);

  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");
  let s = format!("{}-{}-{}", s1, s2, s3); 
  println!("{}", s);

  let s = s1 + "-" + &s2 + "-" + &s3; // s1 is moved
  println!("{}", s);

  println!("============== /concat ===================");
}

pub fn error() {
  println!("============== /rror ===================");
  // let s1 = String::from("hello");
  // let h = s1[0];   // cant get word by index in RUST!!
  // println!("{}", h);

  println!("============== /error ===================");
}

pub fn get_chars_for_bytes() {
  println!("============== get_chars_for_bytes ===================");
  let hello = "Hello";

  let s = &hello[0..4]; // get firs 4 bytes, ru words coded as 2 bytes!! and give 2 words!!
  println!("{}", s);

  for b in hello.bytes() {
    println!("{}", b);
  }
  
  println!("============== /get_chars_for_bytes ===================");
}

pub fn get_chars( word: &str ) {
  println!("============== get_chars ===================");
  for c in word.chars() {
    println!("{}", c);
  }
  println!("============== /get_chars ===================");
}