#[derive(Debug)]
enum LoginData {
    None,
    Invalid,
    NotRegistered,
    UserName(String)
}

fn main() {
    let none_user = LoginData::None;
    println!("{:?}", none_user);
    
    let invalid_user = LoginData::Invalid;
    println!("{:?}", invalid_user);
    
    let not_registered_user = LoginData::NotRegistered;
    println!("{:?}", not_registered_user);
    
    let admin = LoginData::UserName(String::from("Peter Unlustig"));
    println!("{:?}", admin);


}
