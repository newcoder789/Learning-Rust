// Traits: Defining Shared Behavior
// trait defines the functionality a particular type has and can share with other types. We can use
// traits in an abstract way. We can use traits bounds to specify that a generic
// type can be any type that has certain behavior.
// Note: Traits are similar to a feature often called interfaces in other languages. although with
// some differences.


// default traits 
// trait Summary{
//     fn summarize(&self) -> String{
//         format!("Hi there")
//     }
// }

// struct User {
//     name: String,
//     age: u32
// }

// impl Summary for User{
//     fn summarize(&self) -> String{
    //         format!("Hi, my name is {} and I am {} years old", self.name, self.age) 
    //     }
    // }


// traits as parameter
// if we want to have only that can implement summarize
// fn notify(u: impl Summary){
//     println!("{}", u.summarize());
// }

// The Trait syntax works for straightforward cases but is actual syntaxsugar ora longer
// form known as a troit bound; it looks like this:
// ** trait bound ** //
pub fn notify<T, Summary> (item : T){
    println!("Breaking News:", item.summarize());
}
// this means i want to take T as generic which should implement summary property 
// which t generic is bound to traits summary


// fn main(){
//     let user = User{
//         name: "Aryan".to_string(),
//         age: 20
//     };

//     println!("Summary:\n{}",user.summarize() );
// }

