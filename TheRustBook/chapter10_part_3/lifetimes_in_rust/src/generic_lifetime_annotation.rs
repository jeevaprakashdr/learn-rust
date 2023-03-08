

pub fn demo() {
   
   // same life time 
   let first_name = String::from("jeeva");
   let last_name = String::from("prakash");

   let result = longest(first_name.as_str(), last_name.as_str());

   println!("The longest string is {}", result);

    // different life time
    let city_name_1 = String::from("Hubli");
    
    {
        let city_name_2 = String::from("Dharvad");
        let longest_city_name = longest(city_name_1.as_str(), city_name_2.as_str());
        println!("The longest city name is {}", longest_city_name);
    }

    let smallest_city_name ;
    {
        let city_name_2 = String::from("Dharvad");
        smallest_city_name = smallest(city_name_1.as_str(), city_name_2.as_str());
    }
    println!("The smallest city name is {}", smallest_city_name);

}

fn longest<'apple>(s1: &'apple str, s2: &'apple str) -> &'apple str {
    if s1.len() > s2.len()
    {
        s1
    }
    else {
        s2
    }
}
 
fn smallest<'apple>(s1: &'apple str, _s2: &str) -> &'apple str {
    s1
}

