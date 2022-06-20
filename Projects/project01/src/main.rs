use std::io;
use std::cmp::Ordering;

use colored::*;
use rand::{thread_rng, Rng};


macro_rules! get_input {
    ($x:expr) => {
        io::stdin()
            .read_line($x)
            .expect("Fehler beim Einlesen!")
    };
}


fn main() {
    let mut rng = thread_rng();
    println!("{}", "Guessing game".green());

    'main_loop: loop{
        println!("Gib einen Zahlenbereich ein? (n..=m) | n >= 0");

        let n = get_input_number("Gib eine Zahl für n ein:");
        let m = get_input_number("Gib eine Zahl für m ein:");

        
        
        if n > m {
            println!("{}", format!("Zahlenbereich ungültig! {n} > {m}").red());
            continue;
        }

        let mut versuche = 0;
        while versuche < 1 {
            versuche = get_input_number("Gib die Anzahl Versuche ein. Die Zahl muss größer als 0 sein.")
        }

        let number = rng.gen_range(n..=m).wrapping_add(n) % m;

        println!("Es wurde eine Zahl zwischen {} und {} generiert. Errate, welche!", n, m);
        while versuche > 0 {
            println!("Du hast {} Versuche übrig.", versuche);
            let guess = get_input_number("Gib deine Vermutung ab:");
            match guess.cmp(&number) {
                Ordering::Less => println!("{}", "Die gesuchte Zahl ist größer.".red()),
                Ordering::Greater => println!("{}", "Die gesuchte Zahl ist kleiner.".red()),
                _ => {
                    println!("{}", format!("Herzlichen Glückwunsch, die gesuchte Zahl war {number}").green());
                    break;
                }
            };
            versuche -= 1;
        }
        
        println!("Das Spiel ist beendet. Möchtest du erneut spielen? (Y|N)");
        'repeat_loop: loop {
            let mut input = String::new();
            get_input!(&mut input);
            match input.trim() {
                "Y" | "y" => break 'repeat_loop,
                "N" | "n" => break 'main_loop,
                _ => println!("{}", "Ungültige Eingabe, versuche es erneut.".red())
            };
        }
        
    }

    
}

fn get_input_number(prompt: &str) -> usize{
    println!("{}", prompt);
    let mut input = String::new();
    get_input!(&mut input);

    while let Err(_) = input.trim().parse::<usize>(){
        println!("{}", "Ungültige Zahl, versuchen Sie es erneut.".red());
        input = String::new();
        get_input!(&mut input);
    }
    input.trim().parse::<usize>().unwrap()
}