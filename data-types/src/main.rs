fn main() {
    let int : u16 = 12;
    let f32 : f32 = 3.2;
    let bool : bool = true;
    let char : char = 'A';
    let array : [i32;5] = [1,2,3,4,5];
    // this will place 0 in all indexs 
    let array : [i32;1000] = [0 ; 1000 ];
    println!("{}",array[8]);
    let tuple : (&str,&str, i32) = ("Zain","Raza", 2003);
    println!("{} {} was born in {}",tuple.0,tuple.1,tuple.2);
    let slice : &str = "Zain Raza";
    //to convert slice into string
    let str:String = slice.to_string();
    let first_name: &str = "Zain";
    let last_name : &str = "Raza";
    let full_name : String = [first_name , " ", last_name].concat();
    println!("{}",full_name)



}
