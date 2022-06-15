fn main() {
    forer(10);
    whiler(10);
}

fn forer(n: usize){
    for i in 0..n {
        println!("Forer: {}", i);
    }
}

fn whiler(n: usize){
    let mut nums = (0..n).into_iter();
    
    while let Some(num) = nums.next() {
        println!("Whiler: {}", num);
    }
}