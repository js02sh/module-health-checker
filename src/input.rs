use std::io;

pub fn input() ->  String{
    let mut ino = String::new();
    io::stdin().read_line(&mut ino)
        .expect("Faild to read line");
    ino.trim().to_string()
}