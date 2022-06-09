use std::io;


fn main() {
    println!("Please enter a number: ");
    let mut text = String::new();
    io::stdin()
        .read_line(&mut text)
        .expect(&"Failed to read number");
    

    println!("You entered: {}", text);
}
