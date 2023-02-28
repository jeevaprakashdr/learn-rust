mod enums_in_vector; // declaring the module
mod strings_in_rust; 
mod hash_maps;

fn main() {

    // Arrays  
    let _array = [1,2,3];
    // Arrays size is known at the compile time following statement will throw compilation error.
    // println!("the 3rd element is {}", array[8]);
    

    // Vectors are stored on Heep memory and once the scope ended the it will be dropped from the Heep
    // They store one type of data 
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![1,2,3,4,5]; 
    let third = &v2[3];
    println!("the 3rd element is {}", third);
    
    match v2.get(7) {
        Some(third) => println!("element at index {} is {}", 3, third),
        None => println!("there is no element at index {}", 7),
    }

    let mut mutable_vector = vec![1,2,3,4,5];

    // Accessing elements and mutating values 
    println!("original array elements");
    for ele in & mutable_vector {
        println!("square value is {}", ele); // '*' is used to dereference and get only the value from the mutable element.
    }

    for ele in &mut mutable_vector {
        *ele *= 10; // '*' is used to dereference and get only the value from the mutable element.
    }

    println!("after mutating array elements");
    for ele in & mutable_vector {
        println!("square value is {}", ele);
    }

    enums_in_vector::demo_enum_inside_vector();
    strings_in_rust::demo();
    hash_maps::demo();;

}
