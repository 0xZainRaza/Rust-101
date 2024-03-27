enum Status {
    ALIVE,
    DEAD
}

struct Person{
    first_name: String,
    last_name: String,
    Year: u16,
    status: Status 
}



fn main() {
    let person1 = Person{
        first_name : String::from("Zain"),
        last_name : String::from("Raza"),
        Year : 2003,
        status : Status::ALIVE
    };

    println!("{} {} was born in {}",person1.first_name,person1.last_name,person1.Year);
   




}
