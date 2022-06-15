fn main() {
    let closure = |x: i32| -> i32 {x * 2};
    let closure_short = |x| x * 3;
    println!("{}", closure(3));
    println!("{}", closure_short(3));
}
