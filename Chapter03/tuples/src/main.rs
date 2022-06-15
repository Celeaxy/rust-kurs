// Tuples: fixed length, different data types


fn main() {
    let tpl = (123, "Peter Lustig", "😀", b' ');

    println!("{:?}", tpl);
    
    // indexing
    println!("{}", tpl.0);
    println!("{}", tpl.1);
    println!("{}", tpl.2);
    println!("{}", tpl.3);

    // destructuring
    let (_zahl, _name, _smiley, _bytes) = tpl;
}
