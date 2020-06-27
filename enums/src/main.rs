#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddrWithElements {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum MessageDiffDataTypes {
    Quit,  /// is empty
    Move { x: i32, y: i32 }, // has anotation structure 
    Write(String), // has string
    ChangeColor(i32, i32, i32), // has tuple
    Ip( IpAddrKind ) // has struct or another enum
}

impl MessageDiffDataTypes {
    fn call(&self) {
        println!("{:?}", self);
        // method body would be defined here
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrWithElements::V4(String::from("127.0.0.1"));

    println!("{:#?},{:#?}", four, six);
    println!("{:?}",home );

    let m = MessageDiffDataTypes::Write(String::from("hello"));
    m.call();
}

fn route(ip_type: IpAddrKind) {}