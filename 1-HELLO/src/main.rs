fn main() {
    // let x: i32 = 4;
    // println!("{}", is_even(20));
    // println!("{}", fibonachi(6));


    // define a string 
    let s = String::from("Helloooo jugnuuoo");
    let length = str_length(&s);
    println!("NO of char in string is {}", length);
}
// is even
// fn is_even(num: i32)-> bool{
//     if num%2==0{
//         return true;
//     } return false;
// }


// fibonachi series
// fn fibonachi(num: i32)-> i32{
//     let mut first = 0;
//     let mut second = 1;

//     if num == 0{
//         return first;
//     } 
//     if num == 1{
//         return second;
//     }

//     for _ in 1..(num-1){
//         let temp =  second;
//         second = first + second;
//         first = temp;
//     }
//     return second;
// }

fn str_length(str : &str)-> usize{
    // implicit return no need to use return if we do not use ";" and one line code
    str.chars().count()
}