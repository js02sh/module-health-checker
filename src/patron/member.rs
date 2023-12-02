use crate::input::input;
//use crate::patron::{data, member};
use super::linear_rand::rand;
use super::age::ageclac;
use super::data::{checker,database};
// ... (imports and other code)

// ... (imports and other code)

pub fn member_main(check: bool){
    'outer: loop {
        println!("What's your name?:");
        let name = input();
        println!("Hi {}, your Date of Birth: ", name);
        let dob = input(); // Date of Birth
        let age = ageclac(&dob);

        // Use the functions based on the 'check' parameter
        if check {
            // Logging in..
            println!("{}, Please enter your MiRu Id: ", name);
            let member_id: u32 = input().parse().expect("Please input a valid number");

            // Call the checker function and handle the result
            match checker(&name, member_id) {
                Ok(_) => {
                    println!("Login successful!");
                    database(name.clone(), age, member_id, "gen".to_string());
                }
                Err(_) => {
                    println!("User not found. Please try again.");
                    continue 'outer; // Restart the loop if the user is not found
                }
            }
        } else {
            // Signup process
            let mut member_id: u32 = dob.parse().expect("Please write a valid number");
            member_id = rand(&mut member_id);
            println!("What's your gender? (Male: 'm', Female: 'f')");
            let gen = input(); // Gender
            println!("************************************************\n");
            println!(
                "Congratulations {}, we just created your MiRu Id: {}",
                name, member_id
            );
            println!(
                "Name: {}, Id: {}, Age: {}, Gender: {}\n",
                name, member_id, age, gen
            );
            println!("************************************************\n");
            database(name.clone(), age, member_id, "gen".to_string());

            //Call the checker function and handle the result
            // match checker(&name, member_id) {
            //     Ok(_) => {
            //         println!("Signup successful!");
            //         database(name.clone(), age, member_id, "gen".to_string());
            //     }
            //     Err(_) => {
            //         println!("Error creating user. Please try again.");
            //         continue 'outer; // Restart the loop if there is an error creating the user
            //     }
            // }
        };

        // Ask if the user wants to start again
        println!("Do you want to start again? (Yes: 'y', No: 'n')");
        let restart = input();
        match restart.trim() {
            "y" => continue 'outer,
            "n" => {
                println!("Goodbye!");
                return;
            }
            _ => {
                println!("Invalid option. Exiting.");
                break 'outer;

            }
        }
    }
}