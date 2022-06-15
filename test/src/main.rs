use std::time::{Instant, Duration};

fn main() {
    println!("Elapsed: {}ms", measure(sieve).1.as_millis());
}

const LIMIT: usize = 8_000_000;

fn sieve() -> Vec<usize>{
    let mut primes = [true; LIMIT];

   
    primes[0] = false;
    primes[1] = false;
    
    for p in 0..(LIMIT as f64).sqrt().ceil() as usize {
        if primes[p] {
            for i in (p*2 .. LIMIT).step_by(p){
                primes[i] = false;
            }
        }
    }

    primes
        .iter()
        .enumerate()
        .filter(|&(_, is_prime)| *is_prime)
        .map(|(i, _)| i)
        .collect()
}

fn measure<T>(f: fn() -> T) -> (T, Duration){
    let now = Instant::now();
    return (f(), now.elapsed());
}