use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::io;
use std::io::Write;

fn main() -> Result<(), Error> {
    while 1==1 {
    println!("Welcome to my To-Do List\n");

    let mut todolist: Vec<String> = Vec::new();

    // Open the file for reading
    let file = File::open(r"C:\Users\DELL\Documents\Rust-101\Project\src\data.txt")?;
    let reader = BufReader::new(file);

    // Read lines from the file
    for line in reader.lines() {
        let line = line?;
        todolist.push(line);
    }

    for (num, data) in todolist.iter().enumerate() {
        println!("{}. {}", num + 1, data);
    }

    println!("\n[1] Add a To-Do Item.");
    println!("[2] Delete a To-Do Item.");
    println!("[3] Exit.");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    if input.trim() == "1" {
        println!("Enter your To-Do item:");
        let mut additem = String::new();
        io::stdin().read_line(&mut additem)?;

        // Add the new item to the to-do list
        todolist.push(additem.trim().to_string());

        // Open the file in write mode
        let mut file = File::create(r"C:\Users\DELL\Documents\Rust-101\Project\src\data.txt")?;

        // Write each item from the to-do list to the file
        for item in &todolist {
            writeln!(file, "{}", item)?;
        }

        println!("To-Do item added successfully!");
    }


    if input.trim() == "2"{
        println!("Enter the To-Do item number:");
        let mut delitem = String::new();
        io::stdin().read_line(&mut delitem)?;

        todolist.remove(1-1);
        let mut file = File::create(r"C:\Users\DELL\Documents\Rust-101\Project\src\data.txt")?;

        // Write each item from the to-do list to the file
        for item in &todolist {
            writeln!(file, "{}", item)?;
        }
        println!("To-Do item removed successfully!");
    }



    if input.trim() == "3"{
        break;
    } 


    
    }
    Ok(())
}
