use std::mem;

const ARRAY_SIZE: usize = 8000001;
fn main() {
    let xs = [0,1,2,3,4,5];
    println!("data size: {}", mem::size_of_val(&xs));
    println!("{:?}", &xs[3..5]);
    println!("{:?}", &xs[3..=5]);
    print_slice(&xs[0..4]);
}


fn print_slice(slice: &[isize]){
    println!("{:?}", slice);
}