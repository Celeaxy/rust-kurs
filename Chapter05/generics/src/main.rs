struct Data<T>{
    value: T,

}

impl<T> Data<T>{
    fn print_me(&self){
        println!("Hello, it's me!");
    }
}
fn main() {
    let data = Data {value : '2'};
    let data2 = Data {value: 1.0};
    let data3 = Data {value: 3};
    data.print_me();
}
