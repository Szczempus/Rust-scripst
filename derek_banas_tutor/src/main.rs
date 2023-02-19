#![allow(unused)]


use rand::Rng;
use std::arch::x86_64::_CMP_TRUE_UQ;
use std::cmp::Ordering;
use std::fs::File;
use std::{io, vec};
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {

        println!("Number of logical cores is {}", num_cpus::get());
    

    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2 = vec![1,2,3,4];
    // vec2.push(5);
    // println!("1st: {}", vec2[0]);
    // let second = &vec2[1];
    // match vec2.get(1) {
    //     Some(second) => println!("2nd : {}", second),
    //     None => println!("No 2nd value"),
    // }
    // for i in &mut vec2{
    //     *i *= 2;
    // }
    // for i in &vec2 {
    //     println!("{}", i );
    // }

    // println!("Vec len {}", vec2.len());
    // println!("Pop : {:?}", vec2.pop());


//     enum Day {
//         Monday, 
//         Tueseday,
//         Wednesday,
//         Thursday, 
//         Friday,
//         Saturday, 
//         Sunday
//     }

//     impl Day {
//         fn is_weekend(&self) -> bool {
//             match self {
//                 Day::Saturday | Day::Sunday => true,
//                 _ => false
//             }
//         }
//     }
    
//     let today:Day = Day::Sunday;
//     match today {
//         Day::Monday => println!("Monday"),
//         Day::Tueseday => println!("Tueseday"),
//         Day::Wednesday => println!("Wednesday"),
//         Day::Thursday => println!("Thursday"),
//         Day::Friday => println!("Friday"),
//         Day::Saturday => println!("Saturday"),
//         Day::Sunday => println!("Sunday"),
//     }

//     println!("Is today weekeend? {}", today.is_weekend()
// );

    // let st3 = String::from("a k u p s t k a e f");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1{
    //     println!("{}", char);
    // }
    // let st4: &str = "Random String";
    // let mut st5: String = st4.to_string();
    // println!("{}", st5);
    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..6];
    // println!("String len:{}", st6.len());
    // st5.clear();
    // let st6 = String::from("Just some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7;

    // for char in st8.bytes(){
    //     println!("{}", char);
    // }

    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str(" word");
    // for  word in st1.split_whitespace(){
    //     println!("{}", word);
    // }
    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    // let my_tupel: (u8, String, f64) = (47, "Krzysztof".to_string(), 50_000.00);

    // println!("Name: {}", my_tupel.1);
    // let(v1, v2, v3)= my_tupel;
    // println!("Age: {}", v1);

    // let arr_1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    // println!("1st: {}", arr_1[0]);

    // println!("len: {}", arr_1.len());

    // let mut loop_idx = 0;

    // for val in arr_1.iter(){
    //     println!("Val: {}", val);
    // }


    // while loop_idx < arr_1.len(){
    //     println!("Arr:{}", arr_1[loop_idx] );

    //     loop_idx+=1;
    // }

    // loop {
    //     if arr_1[loop_idx] % 2 == 0 {
    //         loop_idx += 1;
    //         continue;
    //     }
    //     if arr_1[loop_idx] == 9 {
    //         break;
    //     }
    //     println!("Val: {}", arr_1[loop_idx]);
    //     loop_idx += 1;
    // }

    //    let my_age = 18;
    //    let votig_age = 18;
    //    match my_age.cmp(&votig_age){
    //        Ordering::Less => println!("Cant Vote"),
    //        Ordering::Greater => println!("Can Vote"),
    //        Ordering::Equal => println!("You Gained THER RIGHT ADGE"),
    // };

    // let age2 = 8;
    // match age2 {
    //     1..=18 => println!("Important  Birthadyt"),
    //     21 | 50 => println!("Important Birthday"),
    //     65..=i32::MAX =>println!("Importatn Birthday"),
    //     _ => println!("Not an Impartant Birthday"), // _ everything else
    // };

    // let mut my_age = 47;
    // let can_vote = if my_age >= 18{
    //     true
    // }else{
    //     false
    // };

    // println!("Can vote: {}", can_vote);

    // let  random_num  = rand::thread_rng().gen_range(1..101);
    // println!("Random num: {}", random_num);

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
