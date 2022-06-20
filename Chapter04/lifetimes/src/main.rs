// every reference has a life time
// life time annotations do not modify the lifetime
// 'a: lifetime (spoken: "tick a")

fn main() {
    // let r: &str;

    // {
    //     let string1 = String::from("Test");
    //     r = &string1;
    // }

    // println!("{}", r);

    let string1 = String::from("Test1");
    let string2 = String::from("Test2");
    let result = longest_string(&string1, &string2);
    
    println!("{}", result);
}

fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() < y.len() {
        y
    } else {
        x
    }
}