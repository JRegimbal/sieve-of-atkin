extern crate atkin;

use atkin::sieve::SieveOfAtkin;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("An argument for the sieve limit is required.");
        return;
    }
    
    let n: u64 = match args[1].parse::<u64>() {
        Ok(x)   => x,
        _       => { println!("Invalid input."); return },
    };
    
    let mut sieve = SieveOfAtkin::new(n);
    sieve.run();
    for prime in &sieve.get_results() {
        print!("{}\t", prime);
    }
    println!();
}
