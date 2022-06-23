fn main() {
    let mut nums = vec![1, 2, 3];

    nums.push(4);
    nums.insert(2, 1);
    pop_empty(&mut nums);
    
    let mut empty = vec![0;0];
    pop_empty(&mut empty);

    let mut nums2 = vec![5,4,3,2,1];

    println!("{:?}", nums2.get(1));
    println!("{}", nums[1]); // may panic if out of bounds
}
fn pop_empty(vec: &mut Vec<i32>) {
    println!("{:?}", vec);
    let popped_val = vec.pop();
    match popped_val {
        Some(n) => println!("Popped first value: {}:", n),
        None => println!("Popped no value!"),
    };
    while let Some(n) = vec.pop() {
        println!("Popped another value: {}", n);
    }
}
