use std::rc::Rc;

#[derive(Debug)]
struct Car {
    name: String,
    year: u16,
    hp: u16,
    mileage: u128
}

impl Drop for Car {
    fn drop(&mut self){
        println!("Calling drop for car with name {}.", self.name);
    }
}
fn box_example(){
    let car1 = Box::new(Car {
        name: String::from("Audi RS3"),
        year: 2022,
        hp: 400,
        mileage: 0
    });

    let car2 = car1;
    // println!("{:#?}", car1); // car1 moved
    println!("{:#?}", car2);
}

fn rc_example(){
    let car1 = Rc::new(Car {
        name: String::from("Audi RS3"),
        year: 2022,
        hp: 400,
        mileage: 0
    });

    {
        let car2 = car1.clone();
        println!("{:#?}", car2);
    }
    
    println!("{:#?}", car1);
}

fn main() {
    // box_example();
    rc_example();
}
