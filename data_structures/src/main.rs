struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

#[derive(Debug)]
struct Rectangle {
    length: u32, 
    width: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }
}

fn main() {
    // let rect: (u32, u32 ) = ( 50, 30 );
    let rect = Rectangle { /// can moved from scope
        length: 50, width: 30
    };
    // let rect = &(Rectangle { length: 50, width: 30 });   // not moved

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
        // area( rect )
    );
    println!("rect1 is {:#?}", rect);

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };
    println!("area of rect1 = {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(50);
    println!("area of rect4 = {}", rect4.area());
}

// tuples example
//fn area( rect: ( u32, u32 ) ) -> u32 {
//     rect.0 * rect.1
// }
// fn area( rect: &Rectangle ) -> u32 {
//     rect.length * rect.width
// }