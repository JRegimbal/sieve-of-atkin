mod atkin {
    extern crate std;
    #[derive(Copy)]
    #[derive(Eq)]
    struct TestInteger {
        value: u64,
        is_prime: bool
    }

    impl Clone for TestInteger {
        fn clone(&self) -> TestInteger { *self }
    }

    impl Ord for TestInteger {
        fn cmp(&self, other: &TestInteger) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }

    impl PartialOrd for TestInteger {
        fn partial_cmp(&self, other: &TestInteger) -> std::option::Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for TestInteger {
        fn eq(&self, other: &TestInteger) -> bool {
            (self.value == other.value) && (self.is_prime == other.is_prime)
        }
    }

    impl TestInteger {
        pub fn new(value: u64) -> TestInteger {
            TestInteger { value: value, is_prime: false }
        }

        pub fn is_prime(&self) -> bool {
            self.is_prime
        }

        pub fn value(&self) -> u64 {
            self.value
        }

        pub fn set_prime(&mut self, prime: bool) -> bool {
            self.is_prime = prime;
            self.is_prime
        }

        pub fn flip(&mut self) -> bool {
            self.is_prime ^= true;
            self.is_prime
        }
    }

    pub struct SieveOfAtkin {
        results: Vec<u64>,
        tests: Vec<TestInteger>,
    }

    enum AtkinCases {
        C1,
        C2,
        C3,
        C4,
    }

    impl SieveOfAtkin {
        pub fn new(limit: u64) -> SieveOfAtkin {
            let res = vec![2,3,5];
            let mut tests: Vec<TestInteger> = Vec::new();
            for i in 2..limit {
                tests.push(TestInteger::new(i));
            }
            SieveOfAtkin { results: res, tests: tests }
        }
        
        // returns a copy so we don't need to worry about our vector being mutated
        pub fn get_results(&self) -> Vec<u64> {
            self.results.clone()
        }

        fn process_case(&mut self, n : u64, index : usize, case: AtkinCases) {
            //Case 1: flip each solution to 4x^2 + y^2 = n
            //Case 2: flip each solution to 3x^2 + y^2 = n
            //Case 3: flip each solution to 3x^2 - y^2 = n where x > y
            //Case 4: do nothing
            match case {
                AtkinCases::C4 => return,
                _ => (),
            }
            let (coefficient_x, coefficient_y): (u64, u64) = match case {
                AtkinCases::C1 => (4, 1),
                AtkinCases::C2 => (3, 1),
                _ => (3, 1),
            };
            match case {
                AtkinCases::C3 =>
                    for i in 1..n {
                        for j in 1..i {
                            if coefficient_x*i*i - coefficient_y*j*j == n {
                                self.tests.get_mut(index).unwrap().flip();
                            }
                        }
                    },
                _ => 
                    for i in 1..n {
                        for j in 1..n {
                            if coefficient_x*i*i + coefficient_y*j*j == n {
                                self.tests.get_mut(index).unwrap().flip();
                            }
                        }
                    },
       
            }
        }

        pub fn run(&mut self) {
            // process test integers for different cases
            for i in 0..self.tests.len()-1 {
                let val = self.tests.get(i).unwrap().clone();
                let case: AtkinCases = match val.value() % 60 {
                    1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => AtkinCases::C1,
                    7 | 19 | 31 | 43 => AtkinCases::C2,
                    11 | 23 | 47 | 59 => AtkinCases::C3,
                    _ => AtkinCases::C4,
                };
                self.process_case(val.value(), i, case);
            }
            
            // start sieving
            while !self.tests.is_empty() {
                let first = self.tests.first().unwrap().clone();
                if first.is_prime() {
                    self.results.push(first.value());
                    let square = first.value() * first.value();
                    let naturals = std::ops::RangeFrom{ start: 1 };
                    for i in naturals {
                        let n_square = i*square;
                        if n_square > self.tests.last().unwrap().value() {
                            break;
                        }
                        let res = self.tests.binary_search(&TestInteger{ value: n_square, is_prime: true });
                        if res.is_ok() {
                            self.tests.get_mut(res.unwrap()).unwrap().set_prime(false);
                        }
                    }
                }
                self.tests.remove(0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use atkin::*;
    #[test]
    fn atkin_test() {
        let primes_to_50 = [2u64, 3u64, 5u64, 7u64, 11u64, 13u64, 17u64, 19u64, 23u64, 29u64, 31u64, 37u64, 41u64, 43u64, 47u64];
        let mut sieve = SieveOfAtkin::new(50);

        sieve.run();
        assert_eq!(sieve.get_results(), primes_to_50);
    }

    #[test]
    fn atkin_output_1000() {
        let mut sieve = SieveOfAtkin::new(1000);
        sieve.run();
        let results = sieve.get_results();
        for prime in &results {
           print!("{} ", prime);
        }
        println!();
    }
}