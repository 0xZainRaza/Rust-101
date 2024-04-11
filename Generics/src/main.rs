fn add_numbers<T: std::ops::Add<Output = T>>(num1: T, num2: T) -> T {
    num1 + num2
}

fn main(){
    let result = add_numbers(1773,288282);
    println!("the result: {}",result);
}

fn add_numbers(num1: u8,num2: u8)->u8{
    num1 + num2
}