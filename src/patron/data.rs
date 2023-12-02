use std::collections::HashMap;
use crate::body::bmi_calc::bodycheck;
use crate::input;

pub static mut PERSON_MAP: Option<HashMap<String, PersonInfo>> = None;

pub fn initialize_person_map() {
    unsafe {
        PERSON_MAP = Some(HashMap::new());
    }
}


#[derive(Debug, Hash, PartialEq, Eq)]
pub struct PersonInfo {
    name: String,
    member_id: u32,
    age: u32,
    gender: String,
    measurements: Vec<(String, u64, u64, u64)>,
}

fn float_to_u64(value: f64) -> u64 {
    (value * 100.0) as u64
}

impl PersonInfo {
    fn new(name: String, age: u32, member_id: u32, gen: String) -> PersonInfo{
        let measurements = vec![bodycheck(name.clone())]; // assuming bodycheck returns a tuple
        PersonInfo { 
            name: name, 
            member_id: member_id, 
            age: age, 
            gender: gen, 
            measurements, 
        }
    }

    fn update_info(&mut self, name: String) {
        let new_measurement = bodycheck(name);
        self.measurements.push(new_measurement);
    }

    fn print_info(&self) {
        println!("Name: {}", self.name);
        println!("Member ID: {}", self.member_id);
        println!("Age: {}", self.age);
        println!("Gender: {}", self.gender);

        // Printing measurements
        for (date, height, weight, bmi) in &self.measurements {
            println!(
                "{} - Height: {:.2} - Weight: {:.2} - BMI: {:.2}",
                date,
                *height as f64,
                *weight as f64,
                *bmi as f64,
            );
        }
    }
}

//Sample data generator
fn create_sample_data() -> (String, PersonInfo) {
    let key = ("Alice".to_string(), 123456);
    let value = PersonInfo {
        name: "Alice".to_string(),
        member_id: 123456,
        age: 25,
        gender: "Female".to_string(),
        measurements: vec![
            ("2023-01-01".to_string(), float_to_u64(160.0), float_to_u64(50.0), float_to_u64(22.0)),
            ("2023-02-01".to_string(), float_to_u64(162.0), float_to_u64(52.0), float_to_u64(23.0)),
        ],
    };
    (key.0.clone(), value)
}

// ... (imports and other code)
// ... (imports and other code)

fn add_sample_data() {
    unsafe {
        let mut person_map = PERSON_MAP.take().unwrap_or_else(HashMap::new);

        // Sample data
        let (sample_key, sample_value) = create_sample_data();
        person_map.insert(sample_key.clone(), sample_value);

        // After adding sample data, update the static variable
        PERSON_MAP = Some(person_map);
    }
}

pub fn checker(name: &str, member_id: u32) -> Result<(), ()> {
    add_sample_data();
    unsafe {
        if let Some(person_map) = PERSON_MAP.as_ref() {
            let key = (name.to_string(), member_id);
            if let Some(info) = person_map.get(&key.0) {
                println!("User found:");
                info.print_info();
                Ok(())
            } else {
                println!("User not found.");
                Err(())
            }
        } else {
            println!("Database not initialized.");
            Err(())
        }
    }
}

pub fn database(name: String, age: u32, member_id: u32, gen: String) {
    unsafe {
        let mut person_map = PERSON_MAP.take().unwrap_or_else(HashMap::new);

        // Sample data
        let (sample_key, sample_value) = create_sample_data();
        person_map.insert(sample_key.clone(), sample_value);

        loop {
            let key = (name.clone(), member_id);
            if let Some(info) = person_map.get_mut(&key.0) {
                println!("Your previous data:");
                info.print_info();
                println!("Would you like to add new data?:");
                println!("Yes: y, No: n");
                let ans = input();
                match ans.trim() {
                    "y" => {
                        info.update_info(name.clone());
                        info.print_info();
                    }
                    "n" => {
                        println!("Bye");
                        break; // Exit the loop
                    }
                    _ => println!("Invalid option"),
                }
            } else {
                println!("We need to add new data today");
                let key2 = key.clone();
                let new_person = PersonInfo::new(name.clone(), age, member_id, gen.clone());
                person_map.insert(key.0.clone(), new_person);
                if let Some(info) = person_map.get(&key2.0) {
                    info.print_info();
                    println!("\n");
                }

                
            }
        }

        // After the loop, update the static variable
        PERSON_MAP = Some(person_map);
    }
}