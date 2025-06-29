//                                              MULTITHREADING 
// In most current operating systems, an executed program's code is run in a process,
// and the operating system will manage multiple processes at once. Within a program,
// you can also have independent parts that run simultaneously. The features that run these
// independent parts are called threads. For example, a web server could have multiple
// threads so that it could respond to more than
// one request at the same time.



use std::thread;
// use std::time::Duration;


// fn main(){
//     let handle = thread::spawn(|| {
//         for i in 1..10{
//             println!("from thread spawn {} ",i );
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     // if u want to sepearte the thread execution
//     handle.join().unwrap();

//     for i in 1..10{
//         println!("from main thread: {}", i);
//     }
// }


// ** Using move Closures with Threads ** //
// We'll often use the move keyword with closures passed to thread: : spawn because the closure will
// then take ownership of the values it uses from the environment, thus transferring ownership of
// those values from one thread to another. In the "Capturing References or Moving Ownership"
// section of Chapter 13, we discussed move in the context of closures. Now, we'll concentrate more on
// the interaction between move and thread: :spawn .

fn main(){
    let v = vec![1,3,4];
    
        // This code doesn't compile
        // Because •v' could go out of scope before the thread starts
        //     This code does compile
        // Because we •move' v to the spawned thread
        // It can not be used in the main thread anymore
    let handle = thread::spawn(move||{
        println!("From vector inside thread: {v:?}");
    });
    handle.join().unwrap();
}
