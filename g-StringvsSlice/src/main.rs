// The String type, which is provided by Rust's standard library rather than coded into the core
// language. is a growable. mutable. owned, IJTF.8 encoded string type. When Rustaceans refer to
// •strings- in Rust, they might be referring to either the String or the string slice astr types, notjust
// one of those types. Although this section is largely about String. both types are used heavily in
// Rust's standard library. and both String and string slices are UTF-8 encoded.


// Slices let you reference a contiguous sequence of elements in a collection rather than the whole
// collection. A slice is a kind of reference, so it does not have ownership.



// Q: Write a function that takes a string as an input
// And returns the first word from it

// Approach #1 • Return a new string
// Problem -
// 1. We take up double the memory
// 2. If the •name• string gets 'cleared ,
// ans• still has *hello' as the value in it

// What wo want is a •view' into tho original string
// And not copy it over

fn main() {
    let me = String::from("Aryan Dixit");
    let ans = first_letter(&me);
    println!("First word: {}", ans);
    
}

fn first_letter(str: &String)-> &str{
    let mut index = 0;
    for (_,c) in str.chars().enumerate(){
        if c == ' '{
            break;
        }
        index = index + 1;
    }
    &str[0..index]
}



