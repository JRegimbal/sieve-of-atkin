extern crate atkin;
use atkin::atkin::SieveOfAtkin;

fn main() {
    let mut sieve = SieveOfAtkin::new(100000);
    sieve.run();
    let results = sieve.get_results();
    for prime in &results {
        print!("{} ", prime);
    }
    println!();
}
