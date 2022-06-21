struct Data<T>{
    _value: T,

}

impl<T> Data<T>{
    fn print_me(&self){
        println!("Hello, it's me!");
    }
}
fn main() {
    let data = Data {_value : '2'};
    let _data2 = Data {_value: 1.0};
    let _data3 = Data {_value: 3};
    data.print_me();
}
