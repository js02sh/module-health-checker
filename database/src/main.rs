use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
struct PersonInfo {
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
                *height as f64 / 100.0,
                *weight as f64 / 100.0,
                *bmi as f64 / 100.0
            );
        }
    }
}

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

pub fn database(name: String, member_id: u32) {
    let mut person_map = HashMap::new();
    let (key, value) = create_sample_data();

    person_map.insert(key.clone(), value);

    // Accessing the value using the same key
    if let Some(info) = person_map.get(&key) {
        info.print_info();
    }
}

fn main() {
    database("Alice".to_string(), 123456);
}
