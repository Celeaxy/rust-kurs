use std::fs::File;
use std::io::{Error, ErrorKind, Read};

fn main(){
    let mut f = match File::open("Cargo.toml"){
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => panic!("YOUR MOM IS NOT HOT!"),
            _ => panic!("Unspecified Error!!!")
        }
    };
    let mut result= String::new();
    f.read_to_string(&mut result);
    println!("{}", result);
}
