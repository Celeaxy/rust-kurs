use std::collections::HashSet;

#[derive(PartialEq, Eq, Debug, Hash)]
struct Data{
    value: i32
}

fn main() {
    let mut hset = HashSet::from([
        (Data {value: 2}), (Data {value: 3})
    ]);

    println!("{:?}", hset);
    println!("{}", hset.contains(&Data{value:3}));
    println!("{}", hset.contains(&Data{value:4}));
    hset.insert(Data{value:4});
    println!("{}", hset.contains(&Data{value:4}));
   
    println!("{:?}", hset);
    hset.remove(&Data{value:3});
    println!("{:?}", hset);

    let set1 = HashSet::from([1,2,3,4]);
    let set2 = HashSet::from([3,4,5,6]);
    let intersect = &set1 & &set2;
    let union = (&set1) | (&set2);
    let difference = &union - &intersect;
    println!("Set1: {:?}", set1);
    println!("Set2: {:?}", set2);
    println!("Intersection: {:?}", intersect);
    println!("Union: {:?}", union);
    println!("Difference: {:?}", difference);
}
