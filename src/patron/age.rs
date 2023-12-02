use chrono::prelude::*;

pub fn ageclac(dob: &str) -> u32 {
    // Get today's date
    let today = Local::now().naive_local().date();

    // Parse the date of birth
    let dob = NaiveDate::parse_from_str(&dob.trim(), "%Y%m%d")
        .expect("Please enter a valid date");

    // Calculate age
    let age = today.year() as u32 - dob.year() as u32;

    // Check if the birthday has occurred this year
    let has_birthday_occurred = today.month() > dob.month()
        || (today.month() == dob.month() && today.day() >= dob.day());

    // Adjust age based on whether the birthday has occurred this year
    if has_birthday_occurred {
        age
    } else {
        age - 1
    }
}
