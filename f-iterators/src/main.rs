// The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is
// responsible for the logic of iterating over each item and determining when the sequence has
// finished. When you use iterators, you dont have to reimplement that logic yourself.
// In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the
// iterator to use it up. For example, the code in Listing 13-10 creates an iterator over the items in the
// vector v1 by calling the iter method defined on . This code by itself doesn't do anything
// useful.
// let v1 = vec![1,3,4];
// let v1_iter = v1.iter();


fn main() {



    //  1. Iterating using for loops
    // let  nums = vec![1,2,3];    
    // for value in nums {
    //     println!("value:{}", value);
    // }


    
    // The iter() method in Rust  provides a way to iterat over the elements of  collection by borrowin them.
    // You can't mutate the variables since we have an immutable reference to the
    // internal elements


    // 2. Iterating after creating itterator
    // let vl = vec![1,2,3];
    // let vl_iter = vl.iter();
    // for val in vl_iter {
    //     println!("value:{}", val);
    // }

    // 3. Iterator using iter_mut();
    let mut vl = vec![1,2,3];
    let vl_iter = vl.iter_mut();
    for val in vl_iter {
        *val = *val + 1;
        println!("value:{}", val);
    }
    println!("{:?}", vl);

    // 4. Iterator using .next
    // let nums = vec![1,2,3];
    // let mut iter = nums.iter();
    // while let Some(val) = iter.next(){
    //     println!("{}", val);    
    // }


    // The Intolterator trait is used to convert a collection into an iterator that takes ~~ownership~~ of the collection. Useful when 
    // 1. You no longer need the original collection 
    // 2. When you need to squeeze performance benefits by transferring ownership (avoiding references)
    //5. IntoIter
    // let nums = vec![1,3,4];
    // let v1_iter = nums.into_iter();
    // for val in  v1_iter{
    //     println!("value:{val}")
    // }




    //     Consuming adapters
    // Methods that call next are called consuming adaptors, because calling them uses up the iterator.

    let nums = vec![1,3,4];
    let v1_iter = nums.iter();
    let sum: i32 = v1_iter.sum();
    println!("Sum:{sum}");
    // cant use it anymore
    // for i in v1_iter{
    //     println!("{i}")
    // }



    // Iterator adaptors re methods defined on the Iterator trait that don't consume the
    // iterator. Instead, they produce different iterators by changing some aspect of the
    // original iterator.

    // let values = vec![1,2,4];
    // let value1 = values.iter();
    // let value2 = value1.map(|x| x+1);

    // for i in value2{
    //     println!("value:{i}");
    // }
    // println!("previous values:{:?}", values);

    //     Assignment -
    // Write the logic to first filter all odd values then double each value and create a new vector 
    let values = vec![1,3,5,6,11,424,64,72,9,13,5634,7];
    let values1 = values.iter().filter(|x| *x%2!=0).map(|x| x*2);
    for value in values1.iter(){
        println!("values: {value}");
    }
    // 
    // 
    //convert itearator back into vector
    //
    //

    let v2: Vec<i32> = values1.collect();
    println!("Final Vector: {:?}", v2)
}
