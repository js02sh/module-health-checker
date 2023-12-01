use std::io;

mod patron;
mod input;
use input::input;
use crate::patron::member::member_main;



fn main () {
    'loop_1: loop {
        println!("*******************************");
        println!("Welcome to MiRu Gym!");
        println!("If you're MiRu Member: 'y' // If Not yet: 'n' "); //check membership
        let mut mem = String::new();
        io::stdin().read_line(&mut mem)
            .expect("Faild to read line");

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
                    break 'loop_1;
                } else {
                    println!("Please input valid input");
                    continue;
                }
            }
            _ => continue
        }
    }
}


// fn main() {
//     // 구조체 초기화 --- (*2)
//     println!("Type your name: ");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name)
//         .expect("Faild to read line");
//     let name = name.trim().to_string();

//     println!("Write your Height:");
//     let (mut height, mut weight) = (String::new(), String::new());
//     io::stdin().read_line(&mut height)
//         .expect("Faild to read line");
//     println!("Write your Weight:");
//     io::stdin().read_line(&mut weight)
//         .expect("Faild to read line");
//     let (height, weight) = (
//             height.trim().parse::<f64>().expect("Please Write valid number"),
//             weight.trim().parse::<f64>().expect("Please Write valid number")
//     );
    

//     let patron = Body::new(name, weight, height);
//     patron.show();

// }

