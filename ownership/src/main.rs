fn main() {
    let mut _message1 = String::from("Hello");
    println!("{}",_message1);

    let _message2 = &mut _message1;
    *_message2 = String::from("Zain");

    print!("{}",_message1);
}