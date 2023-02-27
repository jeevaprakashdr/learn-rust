
struct User {
    username: String,
    email_address: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// Implementation blocks allow us to define/group functions related to struct
// structs allow multiple implementation blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

// associate functions: these are not tight to instance off struct.
impl Rectangle {
    fn squair(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let mut _first_user: User = User{
        username: String::from("jack"),
        email_address: String::from("jack@learnrust.com"),
        active: true,
        sign_in_count: 1
    };

    let _name = _first_user.username;
    _first_user.username = String::from("Jack well");

    let _second_user = build_user(String::from("john@learnrust.com"), String:: from("john"));

    let _user_2 = User {
        email_address: String::from("harry@learnrust.com"),
        username:String::from("harry"),
        .._second_user
    };

    // tuple structs : structs without name fields
    struct Color(i32, i32, i32);

    struct Point(i32, i32, i32); 

    let rect: Rectangle = Rectangle {
        width: 40,
        height: 30
    };

    println!("The area of a rectangle {}", calculate_area(&rect));

    println!("The area of a rectangle through instance method {}", rect.area());

    //  Display traits 
    println!("rect: {:#?}", rect);


    let small_rect: Rectangle = Rectangle {
        width: 20,
        height: 10
    };

    let big_rect: Rectangle = Rectangle {
        width: 60,
        height: 50
    };

    println!("rect can hold small_rect {}", rect.can_hold(&small_rect));
    println!("rect can hold big_rect {}", rect.can_hold(&big_rect));

    // associate functions is invoked to create a new rectangle
     let squired_rect = Rectangle::squair(25);
     println!("squired rectangle {:#?}", squired_rect)

}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email_address: email,
        sign_in_count: 1,
        active: true,
    }
}

fn calculate_area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}