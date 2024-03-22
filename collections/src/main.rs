fn main() {
    println!("Hello, world!");
    let mut _vector : Vec<u8>  = Vec::new();
    _vector.push(1);
    _vector.push(2);
    _vector.push(3);
    _vector.push(4);
    //insert in the middle
    _vector.insert(0, 4);

    _vector.push(5);

    //remove from the index
    _vector.remove(5);

    for data in _vector.iter(){
        print!("{} ",data)
    }
    d =9

    let i: Option<u8>: Option<u8> = _vector.pop();
    match i {
        Some(d) => print!("Hello"),
        None => print!("Not Found")

    }
}
