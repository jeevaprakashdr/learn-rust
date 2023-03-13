use std::{thread, time::Duration, collections::{HashMap}};

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

struct Cacher<T>
where T: Fn(u32) -> u32, // Fn is a trait bound 
{
    calculation: T,
    value: Option<u32>,
    hasmap_value : HashMap<u32, u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None, hasmap_value: HashMap::new() }
    }

    fn value(&mut self, arg: u32) -> u32{
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }

    fn hash_map_value(&mut self, arg: u32) -> u32 {
        match self.hasmap_value.get_key_value(&arg) {
            Some(t) =>  *t.0,
            None => {
                let v = (self.calculation)(arg);
                self.hasmap_value.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = Cacher::new(|num| {
        println!("calculating slowly !!");
        thread::sleep(Duration::from_secs(2)); 
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups", cache_result.hash_map_value(intensity));
        println!("Next, do {} situps", cache_result.hash_map_value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today and rest well");
        }else{
            println!("Today, run for {} minuets", cache_result.value(intensity));
        }
    }
}
