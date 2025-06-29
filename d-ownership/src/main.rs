fn main() {
    println!("Hello, world!");
    // Ownership and Variables.
    // whenever owner goes out of scope, the value is dropped.
    // Ownership is a set of rules that governs how a Rust program manages memory.
    // 1. Each value in Rust has a variable that is called its owner.
    // 2. A value can have only one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    // 4. If the owner is moved to another variable, the original variable can no longer be used.
    // This is called "moving" in Rust.
    // 5. If the owner is copied to another variable, both variables can be used independently.
    // This is called "copying" in Rust.
    // 6. Rust has a concept of "borrowing" which allows you to temporarily use a value without taking ownership.
    // 7. Rust has a concept of "references" which allows you to create a reference to a value without taking ownership.
    // 8. Rust has a concept of "mutable" and "immutable" references which allows you to change a value or not.
    // 9. Rust has a concept of "lifetimes" which allows you to specify how long a reference is valid for.
    // 10. Rust has a concept of "smart pointers" which allows you to create a pointer to a value that has additional functionality.
    // 11. Rust has a concept of "ownership transfer" which allows you to transfer ownership of a value from one variable to another.
    // This is done using the `std::mem::replace` function or by simply assigning the value to another variable.
    // 12. Rust has a concept of "ownership borrowing" which allows you to borrow a value from another variable without taking ownership.
    // This is done using the `&` operator or the `&mut` operator.

    /* 1. 
    let x = 5;
    let y = x;
    println!("Value of no are: {} {}", x,y);
    // rust allows copy of simple variable like int 

    let x: &str = "Hello";
    let y = x;
    println!("Value of no are: {} {}", x,y);

    let x:String = String::from("chlra i guess");
    
    
    // let y = x;
    // it will not work cause rust omoves it by default 
    // println!("change in x and y: ", x,y);
    
    
    let y = x.clone();
    println!("change in x and y: {} {}", x, y);

     */




    /*
    //2 OWNERSHIP AND FUNCTIONS.

    let s: String = String:: from("hello");
    takes_ownership(s);

    //this parameter takes ownership of the value   
    // println!("The length of {}", s);


    // code is copied here
    let no = 4;
    makes_copy(no);
    println!("The square of {} ", no);


    // transfering back the ownership 
    let s = gives_ownership();
    let s2 = String::from("Wodo");
    let s3 = takes_and_gives_back(s2);

    println!("s1 = {}, s3 = {}", s, s3);
     */


    /*3. REference (Borrowing)*/
    //     The Rules of References
    // • At any given time, you can have either one mutable reference or any number of immuta reference .
    // • References must always be valid.
    
    let s1:String = String:: from("hello");
    let len: usize = calculate_length(&s1); // to make it mutable send &mut s1
    println!("The length of ' { } ' is { } . " ,s1, len);

    
}


/* 1-2.fn takes_ownership(s: String) {
    println!("In ownership Function {}", s);
} 
fn makes_copy(no: i32) {
    println!("In makeCOPY: {}", no);
}
fn gives_ownership() -> String {
    let some_string = String::from("JUst DO");
    some_string
    }

fn takes_and_gives_back(str: String) -> String {
    str
 }  // this is a function that takes ownership of the string and returns it back.
 */




 fn calculate_length(s :&String) -> usize {
    let length: usize = s.len();
    length
 }