use regex::Regex;
use colored::*;

use std::env;
use std::fs;

macro_rules! exit {
    ($n:expr) => {
        std::process::exit($n)
    };
}

#[derive(Debug)]
struct Arguments<'a> {
    pattern: &'a String,
    replace: &'a String,
    input_file: &'a String,
    output_file: &'a String
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();

    run(argc, argv);
    
}


fn run(argc: usize, argv: Vec<String>){
    println!("Argc: {}", argc);
    println!("Argv: {:#?}", argv);

    let args = parse_args(argc, &argv);
    let data = read(&args.input_file);
    println!("{}", data);
    let replaced_data = match replace(&args.pattern, &args.replace, &data){
        Ok(text) => text,
        Err(err) => {
            eprintln!("{} {}", "ERROR:".red(), err);
            exit!(1);
        }
    };

    write(&args.output_file, &replaced_data);
}

fn parse_args(argc: usize, argv: &Vec<String>) -> Arguments{
    if argc != 5 {
        eprintln!("{} wrong number of arguments!", "ERROR:".red());
        std::process::exit(1);
    }
    Arguments {
        pattern: &argv[1],
        replace: &argv[2],
        input_file: &argv[3],
        output_file: &argv[4]
    }
}

fn read(file: &String) -> String{
    match fs::read_to_string(file){
        Ok(data) => data,
        Err(err) => {
            eprintln!("Can't read file: {}: {}", file, err.to_string().red());
            std::process::exit(1);
        }
    }
}

fn replace(pattern: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let re = Regex::new(pattern)?;
    
    Ok(re.replace_all(text, replacement).to_string())
}

fn write(file: &str, text: &str){
    match fs::write(file, text){
        Ok(_) => println!("Successfully written to {}", file),
        Err(err) => {
            eprintln!("Error writing to file {}: {}", file, err.to_string().red());
            std::process::exit(1);
        }
    }
}