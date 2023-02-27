
enum Coin {
    Penny,
    Nicket,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    ALabamam,
    Alaska,
    Arizona,
    California
}

fn main() {
    ////////////// Options //////////////////////////

    let _some_number = Some(123);
    let _some_string = Some("a string");

    // type needs to be explicitly inferred for none type 
    let _absent_number: Option<i32> = None;


    let x: i8= 5;
    let y:  Option<i8> = Some(5);

    println!("sum of x and y is {}", y.unwrap_or_default() + x);
    
    value_in_cents(Coin::Quarter(UsState::California));

    println!("sum of option some {}", plus_one(_some_number).unwrap_or_default());
    println!("sum of option none {}", plus_one(_absent_number).unwrap_or_default());
}

////////////// Match expressions //////////////////////////
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Luckky Penny!!");
            return 1;
        },
        Coin::Nicket => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { //// binding patters to value 
            println!("State quarter from {:?}!", state);
            25
        },
    }
 }

 fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
 }