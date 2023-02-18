fn main(){
    let my_string = String::from("Mystrign dfvojkdv.");

    println!("LenghtZ: {}", my_string.len());

    println!("Strinh is empty: {}", my_string.is_empty());

    for token in my_string.split_whitespace(){
        println!("{}", token);
    }

}




/* 

struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn print_description(&self){
        println!("Rectangle {} x {}", self.width, self.height );
    }

    fn is_square(&self) -> bool{
        self.width == self.height
    }
}

fn main(){
    let my_rect = Rectangle { width: 10, height: 10};

    my_rect.print_description();
    
    println!("Retangle is a square {}", my_rect.is_square());

} 

fn main(){
    // let numbers: = [1,2,3,4,5];

    let numbers = [2;400];

    for n in numbers.iter(){
        println!("Value {}", n);
    }

    for i in 0..numbers.len() {
        println!("Value {}", numbers[i]);
    }

} 

struct Color {
    red: u8, 
    green: u8,
    blue: u8, 
}

fn main(){
    let blue = Color{ red: 0, green: 0, blue:  255};

    print_color(&blue);

} 

fn print_color(c: &Color){
    println!("Color - R{}, G{}, B{}", c.red, c.green, c.blue);
}

struct Color(u8, u8, u8);

fn main(){
    let mut red = Color(255, 0, 0);

    red.2 = 60;

    println!("Red is {}, {}, {}", red.0, red.1, red.2);
} 


struct Color {
    red: u8, 
    green: u8,
    blue: u8, 
}

fn main(){
    let mut bg = Color{ red: 255, green: 70, blue:  15};

    bg.blue = 45;

    println!(" {}, {}, {}", bg.red, bg.green, bg.blue)
} 

fn main(){

    let mut x = 10;
    let xr = &x;

    {
        let dom  = &mut x;

        *dom += 1;
    
        println!("X is {}, ",dom );    
    }

    println!("X is {}, ",x );  

} 

fn main(){
    let mut x = 10;
    {
       let x = 15;
    }

    let x = "X is a string";

    println!("x is {}", x);

    let x = true;

    println!("x is {}", x);
} 

fn main(){

   let x = 10;

   {
        let y = 5;

    
   }

   println!("x : {}, y: {}", x, y)
} 

fn main(){

    pritn_numbers_to(20);

} 

fn pritn_numbers_to(num: u32){
    for n in 1..num{
        if is_even(n){
            println!("{} is even", n);
        }

        else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool{
    return num % 2 == 0;
}


fn main(){
    let tup1 = (30,"Kapustka",3.14,false, (1,2,3,4));
    let tup2 = (12,34,"Dorne");
    let (a,b,c) = tup2;

    println!("{}", (tup1.4).3);
    println!("a is {}, b is {}, c is {}", a,b,c);

} 


const MAX_NUM: u8 = 23;

fn main(){
    for n in 1..MAX_NUM{
        println!("{}", n);
    }
}

enum Direction{
    Up,
    Down, 
    Left,
    Right
}

fn main (){

    let player_dirction:Direction = Direction::Up;

    match player_dirction{
        Direction::Up => println!("Dir UP"),
        Direction::Down => println!("Dir Down"),
        Direction::Left => println!("Dir Left"),
        Direction::Right => println!("Dir Right"), 
    }

}

fn main (){
    // let numbers = 30..51;

    let animals = vec!["Rabbit", "Dgo", "cat"];


    for (index, a) in animals.iter().enumerate(){
        println!("The number i is {} and animal is {}", index,a)
    }
}

fn main (){ // while loop
     let mut n = 1;
     while n<=50 {

        if n % 5 == 0{

            println!("n is {}", n);
        }


        n+= 1;
         
     }
}

// fn main(){ // default loop
//     let mut n: i16 = 0;

//     loop {
//         n += 1;

//         if n == 7 {
//             continue;
//         }

//         if n > 10{
//             break;
//         }

//         println!( "The value of n is {}", n);
//     } // break out of  this scope 

// }




// fn main() {

//     // println!("Hello, depod!");

//     // let mut x: u64 = 45;

//     // println!("The value of x is {}", x);

//     // x = 60;

//     // println!("The value of x is {}", x);

//     // let f: f32 = 6.7;

//     // println!("The value of x is {}", x);

//     // let b: bool = true;

//     // let n = 30;

//     // if n < 30{
//     //     println!("The number is less than 30")
//     // }
//     // else if n>30 {
//     //     println!("The number is more than 30")
//     // }
//     // else if n== 30  {
//     //     println!("Rust")
//     // }
// }

*/