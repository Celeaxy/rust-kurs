fn main() {
    println!("Hello, world!");
    checker(1);
    looper(5);
    forer(5);
    whiler(5);
}


fn checker(n: usize){
    println!("{}", match n {
        1 => "Equal",
        m if m > 1 => "Greater",
        _ => "Smaller"
    });
}
fn looper(n: usize){
    let mut counter = 0;

    'loop_label: loop {
        println!("Looper: {}", counter);
        counter += 1;
        if counter == n {
            break 'loop_label;
        }
        
    }
}
fn forer(n: usize){
    for i in 0..n {
        println!("Forer: {}", i);
    }
}
fn whiler(n: usize){
    let mut counter = 0;
    
    while counter < n {
        println!("Looper: {}", counter);
        counter += 1;        
    }
}