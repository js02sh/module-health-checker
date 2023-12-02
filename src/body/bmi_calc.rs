use chrono::prelude::*;
use crate::input;

pub struct Body{
    weight: f64,
    height: f64,
}

impl Body{
    pub fn new(weight:f64, height:f64) -> Body{
        Body { 
            weight,
            height,
            }
    }

    fn calc_bmi(&self) -> f64{
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }
}

pub fn bodycheck(name: String) -> (String, u64, u64, u64){
    let today = Local::now().naive_local().date().to_string();
    println!("Hey {}, What's your current Height & Weight?", name);
    println!("Height:");
    let height: f64 = input().parse().expect("Please input valid number");
    println!("Weight:");
    let weight: f64 = input().parse().expect("Please input valid number");

    let bod = Body::new(weight, height);
    (today, height as u64, weight as u64, bod.calc_bmi() as u64) //return body tuple

}