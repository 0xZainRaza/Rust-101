fn main() {
    let vector = [1,2,3,4,5];    
    for i in 0..=5{
        let option = vector.get(i);
        match option{
            None => println!("No data found at index: {}",i),
            Some(d) => println!("The data at {} index: {}",d,i)
        }

    }



}
