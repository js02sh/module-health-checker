//use std::io;

mod body;
mod patron;
mod input;
use input::input;
use crate::patron::member::member_main;
use crate::patron::data::initialize_person_map;



fn main () {
    initialize_person_map();
        println!("*******************************");
        println!("Welcome to MiRu Gym!");
        println!("If you're MiRu Member: 'y' // If Not yet: 'n' // End: 'q' "); //check membership
        // let mut mem = String::new();
        // io::stdin().read_line(&mut mem)
        //     .expect("Faild to read line");

        let mem = input();


        match mem.trim() {
            "y" => {
                println!("Welcome Back!");
                member_main(true); // send true. go for login
            }
            "n" => {
                println!("Would you like to join Gym MiRu?");
                println!("Yes: 'y', Maybe latter: 'n'");
                let mem = input();
                if mem == "y" {
                    member_main(false); // send false. go for signup
                } else if mem == "n" {
                    println!("Have a nice day, Bye.");
                    //break;
                } else {
                    println!("Please input valid input");
                    //continue;
                }
            }
            "q" => println!("Bye~"),
            _ => println!(".."),
        }
}
