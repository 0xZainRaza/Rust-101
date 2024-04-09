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

impl Person{
    fn kill(&mut self){
        self.status = Status::DEAD;

    }
}

struct Rectangle{
    length: u16,
    width: u16
}

impl Rectangle{
    fn calculate_area(self)-> u16{
        self.length * self.width
    }
}


struct Sheep{
    number: u16
}

trait Animaltraits {
    fn make_noise(&self);
}


impl Animaltraits for Sheep{
    fn make_noise(&self){
        println!("Baa Baa");
    }

}

fn main() {

    //Associated functions
    let mut person1 = Person{
        first_name : String::from("Zain"),
        last_name : String::from("Raza"),
        Year : 2003,
        status : Status::ALIVE
    };

    println!("{} {} was born in {}",person1.first_name,person1.last_name,person1.Year);

    person1.kill();

   match person1.status{
        Status::ALIVE => println!("{} {} is Alive.",person1.first_name,person1.last_name),
        Status::DEAD => println!("{} {} is DEAD.",person1.first_name,person1.last_name)
   }

   let Rectangle1 = Rectangle{
        length: 15,
        width: 20
   };
   
   let area = Rectangle1.calculate_area();
   println!("The area of rectangle is {}",area);
 
   //Traits
   let sheep1 = Sheep{
    number: 1
   };

   sheep1.make_noise();


}
