enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

// enums can have a different values 
enum Message {
    Quit,
    Move {x: i32, y:i32}, // example opf anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32)
}

// eumns with anonymous struct can have functions as bellow  
impl Message {
    fn some_function() {
        println!("rusty you are super cool");
    }
}

fn main() {
    let _four = IpAddressKind::V4;
    let _six = IpAddressKind::V6;

    let locahostV4 = IpAddressKind::V4(192, 168, 0, 1);
    let locahostV6 = IpAddressKind::V6(String::from("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
}

fn route(ip_kind: IpAddressKind) {}