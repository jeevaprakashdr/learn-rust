use std::{thread, time::Duration};

fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly !!");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

fn generate_workout(intensity: i32, random_number: i32) {
    if intensity < 25 {
        println!("Today, do {} pushups", simulated_expensive_calculation(intensity));
        println!("Next, do {} situps", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today and rest well");
        }else{
            println!("Today, run for {} minuets", simulated_expensive_calculation(intensity));
        }
    }
}
