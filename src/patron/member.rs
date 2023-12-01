use std::collections::HashMap;
//use std::io;

use crate::input::input;
use super::linear_rand::rand;
use super::age::ageclac;

// #[derive(Debug, Clone)]  // Add Clone trait derivation
// pub struct User {
//     username: String,
//     user_id: u32,
// }

// impl User {
//     pub fn new(username: &str, user_id: u32) -> Self {
//         User {
//             username: username.to_string(),
//             user_id,
//         }
//     }
// }

fn login(name: String) {
    println!("Hi {}, please enter your MiRu Id: ", name);
    let id = input();
}

fn signup(name: String) {
    println!("Hi {}, what's your Date of Birth: ", name);
    let dob = input(); //Date of Birth
    let age = ageclac(&dob);
    let mut id: u32 = dob.parse().expect("Please Write valid number");
    id = rand(&mut id);
    
    println!("What's your gender?_ (Male: 'm', Female: 'f')");
    let gen = input(); //Gender
    println!("Congratulation {}, we just create your MiRu Id: {}", name, id);
    println!("name: {}, id: {}, age: {}, gender: {}",name, id, age, gen);

}

pub fn member_main(check: bool) {
    println!("May I ask your name?:");
    let name = input();

    // Use the functions based on the 'check' parameter
    if check {
        login(name);
    } else {
        signup(name);
    }
   
}

