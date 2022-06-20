// RAII (Ressource Acquisition Is Initialization)

struct Data;

impl Drop for Data{
    fn drop(&mut self){
        println!("Calling drop for myself");
    }
}
fn main() {
    let box1 = Box::new(5);
    let box2 = Box::new(|| 2);

    println!("{}", *box1+2);
    println!("{}", box2());
    let d = Box::new(Data {});
}
