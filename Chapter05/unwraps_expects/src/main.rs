

fn main() {
    println!("{}", positive(3).unwrap());
    println!("{}", positive(-5).unwrap_or(1));
    println!("{}", positive(-1).unwrap_or_default());
    println!("{}", positive(-1).expect("This panics, because number is below 0!"));
}


fn positive(n: i32) -> Option<i32>{
    if n > 0 {
        return Some(n)
    }

    None
}