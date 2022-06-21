fn main() {
    println!("{}", apply(2, |x| x * 3));
    println!("{}", apply(2, adder));
    println!("{}", apply_differently(2, |x| x * 3));
    println!("{}", apply_differently(2, adder));


    let none: Option<i32> = None;
    none
        .map(|x| x * 3)
        .map(|x| println!("{}", x))
        .expect("NaN!");
}

fn adder(x: i32) -> i32 {
    x + 1337
}

fn apply(x: i32, f: fn(i32) -> i32) -> i32 {
    f(x)
}

fn apply_differently<F>(x: i32, f: F) -> i32
where
    F: Fn(i32) -> i32
{
    f(x)
}