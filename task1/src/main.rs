fn main() {
    // celsium_convert();
    noob_fib();
}

fn celsium_convert() {
    let mut celsium = String::new();
    println!("Enter celsium!");

    std::io::stdin().read_line( &mut celsium ).expect("Error entering");
    let celsium: i32 = celsium.trim().parse().expect("NaN");

    println!("celsium_to_farenheit -> {}", celsium_to_farenheit( celsium ) );
}

fn celsium_to_farenheit( celsium: i32 ) -> i32 {
    ( celsium as f32 * 1.8 ) as i32 + 32
}

fn noob_fib() {
    let mut current = 0;
    let mut prev = 0;

    loop {
        println!("{}", current );

        if current != 0 {
            let tmp = current;
            current = current + prev;
            prev = tmp;
        } else {
            current = 1;
        }
    }
}

