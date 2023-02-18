#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

    let  random_num  = rand::thread_rng().gen_range(1..101);
    println!("Random num: {}", random_num);

    // let num_1:f32 = 1.1111111111111;
    // println!("f32: {}", num_1 + 0.1111111111111);
    // let num_2:f64 = 1.1111111111111;
    // println!("f64: {}", num_2 + 0.1111111111111);


    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);    
    // println!("Max u128 : {}", u128::MAX);

    // const ONE_MIL:u32 = 1_000_000;
    // const PI:f32= 3.141592;
    // let age = "47";
    // let mut age: u32 = age.trim().parse().expect("Age wasnt assigne a numbver");
    // age = age+1; 
    // println!("I'm  {} and I want ${}", age, ONE_MIL);


    // println!("What's your name?");
    // let mut name:String = String::new();
    // let greeting: &str = "Nice to see you";
    // io::stdin().read_line(&mut name).expect("Didn't receive input");

    
    // println!("Hello, {}! {}", name.trim_end(), greeting);


}
