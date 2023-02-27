fn main() {
    
    // ------> ownership rules --------
    // 1. Each value in Rust has a variable that is called its owner.
    // 2. There can only be one owner at a time. 
    // 3. When the owner goes out of scope, the value will be dropped.

    // ------> references rules --------
    // 1. At any given time, you can either have one mutable reference
    // or any number of immutable reference.
    // 2. References must always be valid.

    { // s is not valid here, it is not yet declared
        let s = "hell0";  // s is valid from here
        // 's' is a string literal and store a fixed sized binary

        let dynamic_size_string = String::from("HELLO"); // this variable is dynamic in size and allocated in HEAP memory
        // do some stuffs with 's' and 'dynamic_size_string'
    } // 's' and 'dynamic_size_string' is out of scope and no longer valid. Rust will de-allocate the memory for the variables. 


    {
        let x = 5;
        let y = x; // here the value get copied to new variable

        // In rust the reference variables are not shallow copied. 
        // Rather they are moved.
        let s1 = String::from("Move");
        let s2 = s1; // moved (not shallow copied or pointing to the same reference point)

        // println!("{}", s1); // compiler error happens. Uncomment and check the error.

        // If incase we want to move the value 
        // we can use the more expensive 'Clone' method
        let s3 = String::from("Clone");
        let s4 = s3.clone(); 
        
        println!("{}", s3);
    }

    // variable ownership with functions
    {
        let s = String:: from("hello");

        takes_ownership(s); // this is same as passing 'S' to another variable where it can not be 'Copied', it is just 'MOVED'
    
        // println!("{}", s) HERE an error is thrown as the variable 's' scope ended after passing to function 'takes_ownership'
    }  

    // passing variables with references.
    // this is called BORROWING
    {
        let s = String:: from("rust is great");

        let len = calculate_length(&s); 
    
        println!("length of a string '{}' is {}", s, len); // we can access 'S' as it is passed as reference.
    }


    // passing variables with references.
    // in case we need to change the value of the reference 
    // pass mutable reference
    {
        let mut s = String:: from("rust is great");

        change(&mut s); 
    
        println!("length of a string '{}'", s); 
    }

    {
        // string slice
        let s = "hello world";
        let word = first_word(s);
        println!("sliced first word'{}'", word); 
    }
    
}

fn change(s: &mut String) {
    s.push_str("change value");
}

fn takes_ownership(some_string: String) {
    // this method takes the ownership of 'S' from here on
    println!("{}", some_string)
}

fn calculate_length(s: &String) -> usize {
    // references are immutable by default and we cannot modify
     // s.push_str("change value ") This statement will throw error

    let length = s.len();
    return length;
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..];
}
