fn main() {
    panic_test(3);
}


fn panic_test(n: i32){
    if n < 100 {
        panic!("AAAAAH{}!", 3);
    }
}