pub mod sieve;
mod test_integer;

#[cfg(test)]
mod tests {

    use sieve::SieveOfAtkin;

    #[test]
    fn atkin_test() {
        let primes_to_50 = [2u64, 3u64, 5u64, 7u64, 11u64, 13u64, 17u64, 19u64, 23u64, 29u64, 31u64, 37u64, 41u64, 43u64, 47u64];
        let mut sieve = SieveOfAtkin::new(50);

        sieve.run();
        assert_eq!(sieve.get_results(), primes_to_50);
    }

    #[test]
    fn atkin_output() {
        let mut sieve = SieveOfAtkin::new(10000);
        sieve.run();
        let results = sieve.get_results();
        for prime in results.iter() {
           assert!(is_n_prime(*prime), "Failed with {}", prime);
        }
    }

    fn is_n_prime(n: u64) -> bool {
        let mut prime: bool = true;
        if n > 2 {
            for i in 2..n/2 {
                if n % i == 0 {
                    prime = false;
                    break;
                }
            }
        }
        prime
    }
}
