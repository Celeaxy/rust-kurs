const THRESHOLD: i32 = 10; // wird vor Kompilierung im Code ersetzt durch den Wert
static THRESHOLD2: i32 = 5; // immer die gleiche Speicheradresse

fn is_above_threshold(n: i32) -> bool{
    n > THRESHOLD
}

fn is_above_threshold2(n: i32) -> bool{
    n > THRESHOLD2
}

fn main() {
    let value = 100;
    println!("{}", is_above_threshold(value));
    println!("{}", is_above_threshold2(value));
}
