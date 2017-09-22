extern crate atkin;
use atkin::atkin::SieveOfAtkin;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u64;
    if args.len() < 2 {
        println!("An argument for the sieve limit is required.");
        return;
    }
    match args[1].parse::<u64>() {
        Ok(x) => n = x,
        _ => {println!("Invalid input."); return},
    };
    
    let mut sieve = SieveOfAtkin::new(n);
    sieve.run();
    let results = sieve.get_results();
    for prime in &results {
        print!("{} ", prime);
    }
    println!();
}
