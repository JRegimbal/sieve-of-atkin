struct TestInteger {
    value: u64,
    mut is_prime: bool
}

impl TestInteger {
    fn new(value: u64) -> TestInteger {
        TestInteger { value, false}
    }

    fn is_prime(&self) -> bool {
        self.is_prime
    }

    fn value(&self) -> u64 {
        value
    }

    fn set_prime(&mut self, prime: bool) -> bool {
        self.is_prime = prime;
    }

    fn flip(&mut self) -> bool {
        self.is_prime = !self.is_prime
    }
}

struct SieveOfAtkin {
    mut results: Vec<u64>,
    mut tests: Vec<TestInteger>,
}

enum AtkinCases {
    C1,
    C2,
    C3,
    C4,
}

impl SieveOfAtkin {
    fn new(limit: u64) -> SieveOfAtkin {
        let mut res = vec![2,3,5];
        let mut tests = vec![1..limit];
        SieveOfAtkin { res, tests }
    }
    
    // returns a copy so we don't need to worry about our vector being mutated
    fn get_results(&self) -> Vec<u64> {
        results.clone()
    }

    fn process_case(&mut self, n : u64, case: AtkinCases) {
        //Case 1: flip each solution to 4x^2 + y^2 = n
        //Case 2: flip each solution to 3x^2 + y^2 = n
        //Case 3: flip each solution to 3x^2 - y^2 = n where x > y
        //Case 4: do nothing
    }

    fn run(&mut self) {
        // process test integers for different cases
        for val in self.tests.iter() {
            let case: AtkinCases = match val.value() % 60 {
                1 | 13 | 17 | 29 | 37 | 41 | 49 | 53 => AtkinCases::C1,
                7 | 9 | 31 | 43 => AtkinCases::C2,
                11 | 23 | 47 | 59 => AtkinCases::C3,
                _ => AtkinCases::C4,
            }
            self.process_case(val.value(), case);
        }
        
        // start sieving
        while !self.tests.is_empty() {
            let first = self.tests.first().unwrap();
            if first.is_prime() {
                self.results.append(first.value());
                let square = first.value() * first.value();
                for i in (1..) {
                    let n_square = i*square;
                    if n_square > self.tests.last().unwrap() {
                        break;
                    }
                    self.results.tests.get_mut(n_square - first.value()).set_prime(false);
                }
            }
            self.tests.remove(0);
        }
    }
}
