use std::collections::HashMap;

fn main() {
    let mut hmap: HashMap<i32, &str> = HashMap::new();

    hmap.insert(3, "three");
    hmap.insert(5, "five");

    println!("{:?}", hmap);
    let old = hmap.insert(5, "not five");
    println!("{:?}", hmap);
    match old {
        Some(val) => println!("Old value : {}", val),
        None => {}
    };
    println!("Contains 5?: {}", hmap.contains_key(&5));
    println!("Contains 4?: {}", hmap.contains_key(&4));

    let old2 = hmap.remove(&4);
    match old2 {
        Some(val) => println!("Removed value : {}", val),
        None => println!("Key was not present.")
    };
    let old3 = hmap.remove(&5);
    match old3 {
        Some(val) => println!("Removed value : {}", val),
        None => println!("Key was not present.")
    };
    println!("{:?}", hmap);
    hmap.insert(2, "two");
    for k in hmap.keys(){
        println!("Key: {}", k);
    }
    println!("{:?}", hmap.get(&2));
} 