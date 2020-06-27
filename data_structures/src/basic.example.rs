struct Color(i32, i32, i32); // tuples struct, if we dont ned set names to fields

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn main() {
  let mut user1 = build_user(String::from("someone@example.com"),String::from("someusername123"));
  user1.email = String::from("anotheremail@example.com");
  let mut user2 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
      ..user1 // like spread in js objects
  };
  user1.email = String::from("anotheremail@example.com");
  println!(
      "[{};{};{};{}]",
      user1.username,
      user1.email,
      user1.active,
      user1.sign_in_count
  );
}

fn build_user(email: String, username: String) -> User {
  User {
      email, // like js! if value of struct and fn var is same -> dont need set value
      username: username,
      active: true,
      sign_in_count: 1,
  }
}