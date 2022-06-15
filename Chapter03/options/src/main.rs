fn main() {
    let x: u32 = 5;
    let y: Option<u32> = Some(3);

    println!("{} + {} = {}", x, y.unwrap(), add(x, y));
}

fn add(x: u32, y: Option<u32>) -> u32 {
    match y {
        Some(n) => x + n,
        None => x
    }
}


// enum Option<T>{
//     Some(T),
//     None
// }