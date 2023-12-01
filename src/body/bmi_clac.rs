pub struct Body{
    name : String,
    weight: f64,
    height: f64,
}

impl Body{
    pub fn new(name: String, weight:f64, height:f64) -> Body{
        Body { 
            name,
            weight,
            height,
            }
    }

    fn calc_bmi(&self) -> f64{
        let h = self.height / 100.0;
        self.weight / h.powf(2.0)
    }

    pub fn show(&self){
        println!("{}'s BMI is {:.2}",self.name,self.calc_bmi());
    }
}