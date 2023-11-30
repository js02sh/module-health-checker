mod bmi_checker;
use bmi_checker::bmi_clac::Body;
use std::io;

fn main() {
    // 구조체 초기화 --- (*2)
    println!("Type your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
        .expect("Faild to read line");
    let name = name.trim().to_string();

    println!("Write your Height:");
    let (mut height, mut weight) = (String::new(), String::new());
    io::stdin().read_line(&mut height)
        .expect("Faild to read line");
    println!("Write your Weight:");
    io::stdin().read_line(&mut weight)
        .expect("Faild to read line");
    let (height, weight) = (
            height.trim().parse::<f64>().expect("Please Write valid number"),
            weight.trim().parse::<f64>().expect("Please Write valid number")
    );
    

    let patron = Body::new(name, weight, height);
    patron.show();

}

