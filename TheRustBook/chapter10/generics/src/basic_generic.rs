pub fn basics()
{
    let _number_list_1 = vec![10, 1, 4, 6, 5,3,9];

    let mut largest_number = _number_list_1[0];
    for number in _number_list_1 {
        if number > largest_number {
            largest_number = number;
        }
    }

    println!("largest number in list 1 {}", largest_number);

    let _number_list_2 = vec![10, 1, 22, 4, 6, 5,3,9];

    let mut largest_number = _number_list_2[0];

    for number in _number_list_2 {
        if number > largest_number {
            largest_number = number;
        }
    }

    println!("largest number in list 2 {}", largest_number);

    let _number_list_3 = vec![10, 1, 4, 6, 88, 5,3,9];
    println!("largest number in list 3 from generic function {}", get_largest_number(_number_list_3));

    let _char_list = vec!['j', 'e', 'e', 'v', 'a'];
    println!("largest in char list is {}", get_largest(_char_list));
}

fn get_largest_number(number_list: Vec<i32>) -> i32 {
    let mut largest_number = 0;

    for number in number_list {
        if number > largest_number {
            largest_number = number;
        }
    }

    largest_number
}

fn get_largest<T : PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
