fn main() {
    let name = "Peter Witzig".to_owned();

    
    take_reference(&name);
    take_reference(&name[..5]);
    take_ownership(name);
}

fn take_ownership(s: String){
    println!("{}", s);
}

fn take_reference(s: &str){
    println!("{}", s);
}