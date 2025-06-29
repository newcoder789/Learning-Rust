use::std::collections::HashMap;
// Hashmaps stores a key value pair in rust
// Similar to objects in JS
// Dict in Python
// HashMap in Java

fn convert_to_hasmap(vec: Vec<(String,i32)>)-> HashMap<String,i32> {
    let mut hm = HashMap::new();
    for (key, values) in vec{
        hm.insert(key, values);
    }
    hm
}
pub fn working(){
    // create a function that takes a vector of tuples and returns a hashmap
    let vec: Vec<(String, i32)> = vec![
        (String::from("hary"), 3),
        ("banana".to_string(), 5),
        ("orange".to_string(), 2),
    ];
    let hm  = convert_to_hasmap(vec);
    println!("The hashmap is {:?}", hm);

    
}