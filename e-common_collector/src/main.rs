mod vector;
mod hashmap;
// Rust's standard library includes a number of very useful data structures called collections.
// Most other data types represent one specific value, but collections can contain multiple values.
// the data these collections point to is stored on the heap
// rather than the stack, so they can grow and shrink as needed at runtime.

// first one is vector.
    // Vectors allow you to store more than one
    // value in a single data structure that puts all
    // the values next to each other in memory.


fn main() {
    println!("Hello, world!");

    vector::demonstrate();
    hashmap::working();

 }
