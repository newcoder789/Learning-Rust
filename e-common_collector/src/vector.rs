pub fn demonstrate(){
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("the vector with even no: {:?}", even_filter(&vec));
    println!("The vector is {:?}", vec);

    let _ = [1,2,3];

    // let v:Vec<i32> = Vec::new();  
    // v.push(1);
    // v.push(2);
    {
    let _ = vec![1,2,3];
    }// _ will be droped here 


    /*
    ACCESSING VECTOR
     */

    let v = vec![1,2,3,3,5,6];

    // let third = &v[2];

    // let third = &v[20]; ---- will throw runtime error


    match v.get(20){// get returns options enum
        Some(third) => println!("the no {}", third),
        None=> println!("the no not found ")
    }
    let mut v3 = vec![3;9];
    for i in &mut v3{
        *i +=50;
    }
    for i in &v3{
        println!("Value {}", i)
    }

    

}


fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();
    for &item in vec.iter() {
        if item % 2 == 0 {
            even_vec.push(item);
        }
    }
    even_vec
}