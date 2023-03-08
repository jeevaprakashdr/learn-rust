use std::fmt::Debug;

use crate::trait_implementation::{Summary, Display};

pub fn notify_sugur_coating_of_bellow(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
} 

pub fn notify<T : Summary>(item : &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_with_multiple_impl_parameters(item1: &impl Summary, item2: &impl Summary) {
    // 
} 

pub fn notify_with_multiple_trait_bound<T : Summary>(item1 : &T, item2: &T) {
    //
}


pub fn notify_with_multiple_impl_parameters_and_type(item1: &(impl Summary + Display), item2: &(impl Summary + Display)) {
    // 
} 

pub fn notify_with_multiple_trait_bound_type<T : Summary + Display>(item1 : &T, item2: &T) {
    //
}


pub fn notify_with_improved_readability<T, U>(item1 : &T, item2: &U) 
    where   T: Summary + Display + Clone,
            U: Debug  + Display
{
    //
}