fn main() {
    let s = "Hello World!";

    print!("{}\n", s);
    println!("{}", s);
    
    
    eprint!("{}\n", s);
    eprintln!("{}", s);

    let name = "Peter PP";
    
    // before rust 1.58
    let s2 = format!("Hello {}, nice to meet you!", name);
    println!("{}", s2);

    // since rust 1.58
    let s3 = format!("Hello {name}, nice to meet you!");
    println!("{}", s3);
}
