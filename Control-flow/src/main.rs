

fn add(i1:u8,i2:u8){
    print!("{},{} ",i1,i2);
}

fn add_int(val1:u8,val2:u8)->u8{
    let add_result = val1 + val2;
    return add_result;    
}


fn main() {
    let val1 = 13;
    let val2 = 67;

    if val2 >= val1 {
        print!("The values1 is greater than equal to val2 \n")
    }
    else if val1 > val2 {
        print!("The Value1 is greater than  Value2\n")
    }
    else if val2 > val1{
        print!("The value2 is greater than value2\n")
    }
    else{
        print!("The Values are not equal\n")
    }


    //Match Key word
    let animal: &str = "Do0g";
    match animal {
        "Dog" => println!("Woof\n"),
        _ => println!("Unkown\n") 

    }


    //enum
    enum Gender {
        Male,
        Female
    }

    let (firstname,lastname,gender) = ("Zain","Raza",Gender::Male);

    match gender {
        Gender::Male => println!("{} {}",firstname,lastname),
        Gender::Female => println!("{} {}",firstname,lastname)
    };

    //loops

    for i in 0..=10{
        print!("{}\n",i)
    }

    let array = [1,2,3,4,5];
    for index in 0..array.len(){
        print!("{}",array[index])
    }

    for data in array.iter(){
        print!("{}",data)
    }


    let mut counter = 0;
    while 1==1 {
        print!("count: {}\n",counter);
        counter = counter + 1;
        if counter == 10{
            break;
        }

    }

    loop {
        add(1,2);
        break;
    }

    for i in 0..4{
        let ret = add_int(i, i);
        print!("\nResult = {}",ret);
    }


}
