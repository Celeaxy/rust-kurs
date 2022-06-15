fn main() {
    let f = curried_f(4);
    println!("{:?}", f(3));
}

fn curried_f(inp: i32) -> Box<dyn Fn(i32) -> i32>{
    Box::new(move |x|x*inp)
}



