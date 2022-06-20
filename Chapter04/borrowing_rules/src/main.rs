// As many immutable references as we want, but then no mutable references
// Only one mutable reference, but then no immutable references

fn main() {
    let mut s = "Name".to_owned();

    let _r1 = &s;
    let _r2 = &s;
    // let _r3 = &mut s; // schmeißt einen fehler

    println!("{} {} {}", s, _r1, _r2);
}
