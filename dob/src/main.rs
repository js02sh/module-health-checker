use std::io;
use chrono::prelude::*;

fn ageclac(dob: &str) -> i32{
    // Get today's date
    let today = Local::now().naive_local().date();

    // Get the user's date of birth
    //println!("Enter your date of birth (YYYYMMDD): ");

    // Parse the date of birth
    let dob = NaiveDate::parse_from_str(&dob.trim(), "%Y%m%d")
        .expect("Please enter a valid date");

    // Calculate age
    let age = today.year() - dob.year();

    // Check if the birthday has occurred this year
    let has_birthday_occurred = today.month() > dob.month()
        || (today.month() == dob.month() && today.day() >= dob.day());

    // Adjust age based on whether the birthday has occurred this year
    if has_birthday_occurred { age } else { age - 1 }

}



fn main () {
    let mut age = String::new();
    println!("Enter your date of birth (YYYYMMDD): ");
    io::stdin().read_line(&mut age)
        .expect("Faild to read line");
    let age = ageclac(&age);
    println!("{}",age);

}