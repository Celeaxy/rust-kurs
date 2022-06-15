fn first_word(s: &str) -> &str{
    for (i, &character) in s.as_bytes().iter().enumerate(){
        if character == b' ' {
            return &s[..i];
        }
    }
    return s
}
fn main() {
    let smiley = "😀";
    let sparkle_heart = "💖";

    let mut text = String::from("Rust is fun!");
    text.push_str(&smiley);
    text.push(' ');
    text.push_str(&sparkle_heart);
    println!("{}", text);

    for b in text.bytes(){
        println!("{}", b);
    }
    for c in text.chars(){
        println!("{}", c);
    }

    println!("{}", first_word(&text));
}