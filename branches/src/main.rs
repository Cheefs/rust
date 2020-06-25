fn main() {
    let number = 7;

    if number < 5 { // arguments must be Boolean type
        println!("condition was true");
    } else {
        println!("condition was false");
    }


    let condition = true;
    let number = if condition { // all returns value must be same type
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
