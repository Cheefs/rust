enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
  match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter => 25,
  }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
        _ => (), // like "else" or "default" in switch
    }
}

fn doit(coin:Coin){

  let mut count = 0;

  if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
  } else {
    count += 1;
  }

  println!("{:?}", count);
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}