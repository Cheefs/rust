extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

/**
 * let foo = 5; // immutable
 * let mut bar = 5; // mutable
*/

fn main() {
    println!("Guess the number!");

    let sercret_number = rand::thread_rng().gen_range( 1, 101 );
    println!("sercretNumber is, {}", sercret_number );  

    loop {
        println!("Enter number from {} to {}", 1, 101);
        let mut number = String::new();

        io::stdin()
        .read_line( &mut number)
        .expect("Read line fail");

        let number: u32 = match number.trim().parse() {
            Ok( num ) => num,
            Err( _ ) => {
                println!("not a number!!!");
                continue;
            }
        };

        println!("You guessed, {}", number );

        match number.cmp( &sercret_number ) {
            Ordering::Less => println!("Less"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("WON");
                break;
            }
        }
    }
}