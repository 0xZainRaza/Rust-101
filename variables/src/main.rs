fn main() {
    let _my_integer : i32 = 32;
    const _MY_CONSTANT : i32 = 92 ;
    let int8 : u8 = 255;
    println!("{}",int8);
    let _int16 : u16 = int8 as u16;
    println!("{}",_int16);

    let int16 : u16 = 65535;
    println!("{}",int16);

    let int8 : u8 = int16 as u8;
    println!("{}",int8);

    let int8 : u8 = 65;
    let test : char = int8 as char;
    print!("{}\n",test);

    let mut zain : u8 = 90;
    zain = 99;
    print!("{}",zain);




}
