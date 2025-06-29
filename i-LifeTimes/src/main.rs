//                                                              Lifetimes
//                                                    Lifetimes are hard to digest.
//                                         Takes a lot of time to understand why they are needed
//                                   Lot of times the compiler will help you and guide you in the right direction


// Lets try to understand using an example
// Q - Write a function that takes two strings as an input
// And returns the bigger amongst them


// Lifetime Annotations in Function Signatures
// To use Efetime annotations in function signatures. we need to declare the generic
// parameters inside angfe brackets between the function name and the parameter list, just as we did
// with generic type paramctcrs,
// We want the Osnature to express the following constraint: the retumed reference will be valid as
// long as both the parameters are valid. This is the relationship between lifetimes of the parameters
// and the return value. Well name the lifetime 'a and then add it to each reference. as shown in
// Listing 10-21.
// Filename: src/maån,rs


// here str1 has lifetime from line 34 to 49 
// str 2 has lifetime from line 36 to 38
// so using our method ans has lifetime for the line where both of them are present i.e 36 to 38 only 
// hence line 42 will not be able to print cause our compiler dont know which string might be bigger hence even if str1 is bigger it will throw erro 

// fn biggest_string<'a>(str1: &'a str, str2: &'a str)-> &'a str{
//     if str1.len() > str2.len(){
//         return str1;
//     }
//     return str2;
// }

// fn main() {
//     let ans;
//     let str1 = String::from("adfasfas");
//     {
//         let str2 = String::from ("dsafs");
//         ans = biggest_string(&str1, &str2);
//     }
//     println!("{}", ans);
// }


// struct with lifeTIme
struct User<'a>{
    name: &'a str
}


fn main(){
    let first_name = "Aryan".to_string();
    let user = User{ name: &first_name};
    println!("The name of the User: {}", user.name);

}