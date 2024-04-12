use std::thread;
use std::sync::mpsc;

fn main() {

    let handle = thread::spawn(||{println!("Hello world from thread");});

    let result = handle.join();
    match result {
        Ok(_) => println!("Threading finished"),
        Err(_) => println!("Threading failed ")
    }

    let value = 20;

    let handle1 = thread::spawn(move || -> i32 { value * 2} );
    let result = handle1.join();

    match result{
        Ok(r) => println!("Result: {}",r),
        Err(_) => println!("It didnt happen")
    }

    //channel

    let (s,r) = mpsc::channel();
    let _ = thread::spawn(move || {
        let _ = s.send("Hello from the s");
    });

    match r.recv(){
        Ok(re)=>println!("Result: {}",re),
        Err(_) => {}   
    }




    

}
