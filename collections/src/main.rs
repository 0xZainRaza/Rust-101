

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
    
    let mut test : Vec<u8>  = Vec::new();

    let i: Option<u8> = test.pop();
    match i {
        Some(_d) => println!("Hello"),
        None => println!("Not Found")
    }

    test.push(4);
    let result:bool = test.contains(&4);
    print!("{}\n",result);
    
    let result2:bool = test.contains(&5);
    println!("{}",result2)



}
