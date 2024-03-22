

use std::collections::HashMap;
use std::collections::BTreeMap;

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
    println!("{}",result2);

    //Collections
    let mut _map : HashMap<u8,&str> = HashMap::new();

    _map.insert(1,"zain");
   
    _map.insert(3,"hehe");
    _map.insert(2,"umer");

    for kvp in _map.iter() {

        print!("Key {}, Value {}\n",kvp.0,kvp.1)
        
    }


    let mut _map2: BTreeMap<u8, &str> = BTreeMap::new();

    _map2.insert(1, "Less goo1");
    _map2.insert(3, "Less goo3");
    _map2.insert(2, "Less goo2");
    
    for kvp in _map2.iter(){
        print!("Key = {}, value = {}\n",kvp.0,kvp.1);

    }

    
    let mut _map3 : HashMap<u8,&str> = HashMap::new();

    _map3.insert(1, "Test1");
    _map3.insert(2, "Test2");
    _map3.insert(3, "Test3");



    if _map3.contains_key(&2){
        _map3.insert(2, "Overloaded");
    }

    for kvp in _map3.iter(){
        print!("Key = {}, Value: {}\n",kvp.0,kvp.1);
    }










}
