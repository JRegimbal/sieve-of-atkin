use test_integer::TestInteger;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;
use std::collections::VecDeque;


struct Squares {
    count: u64,
    max: u64,
}

impl Squares {
    pub fn new(max: u64) -> Squares {
        Squares { count: 0, max: max }
    }
}

impl Iterator for Squares {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        let square = self.count*self.count;
        if square > self.max {
            None
        } else {
            Some(square)
        }
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
        if limit < 2 {
            panic!("The limit for the sieve must be above 2. {} was set as limit.", limit);
        } else {

            for i in 2..limit {
                tests.push(TestInteger{ value: i, is_prime: false });
            }
        }
        SieveOfAtkin { results: res, tests: tests }
    }

    // returns a copy so we don't need to worry about our vector being mutated
    pub fn get_results(&self) -> Vec<u64> {
        self.results.clone()
    }

    fn process_case_static(n : u64, index : usize, case: AtkinCases, tests: &mut Vec<TestInteger>) {
        //Case 1: flip for each solution to 4x^2 + y^2 = n
        //Case 2: flip for each solution to 3x^2 + y^2 = n
        //Case 3: flip for each solution to 3x^2 - y^2 = n where x > y
        //Case 4: do nothing

        let (co_x,co_y): (u64, u64) = match case {
            AtkinCases::C1  => (4,1),
            AtkinCases::C2  => (3,1),
            AtkinCases::C3  => (3,1),
            _               => { return; },
        };

        let expression: Box<Fn(u64, u64) -> u64> = match case {
            AtkinCases::C3  => Box::new( |x, y| { if x > y { co_x*x - co_y*y } else { 0 } } ),
            _               => Box::new( |x, y| { co_x*x + co_y*y } ),
        };

        for i in Squares::new(n) {
            for j in Squares::new(n) {
                if expression(i, j) == n {
                    tests.get_mut(index)
                        .expect("Index passed in process is invalid. This shouldn't happen.")
                        .flip();
                }
            }
        }
    }

    pub fn run(&mut self) {
        let mut deque: VecDeque<usize> = VecDeque::new();
        for i in 0..self.tests.len()-1 {
            deque.push_back(i);
        }
        let integers = Arc::new(Mutex::new(deque));
        let mut handles = vec![];
        let mut tests = vec![];
        let (tx, rx) = mpsc::channel();
        // process test integers for different cases
        for _ in 0..10 { //spawns 10 threads
            let integers = integers.clone();
            let mut test = self.tests.clone();
            let t = tx.clone();
            let handle = thread::spawn(move || {
                loop {
                    let integer = integers.lock().unwrap().pop_front();
                    if integer.is_none() {
                        break;
                    }
                    let integer = integer.unwrap();
                    let val = test.get(integer)
                        .expect("Index passed is invalid. This shouldn't happen.")
                        .clone();
                    let case: AtkinCases = match val.value() % 60 {
                        1  | 13 | 17 | 29 | 37 | 41 | 49 | 53   => AtkinCases::C1,
                        7  | 19 | 31 | 43                       => AtkinCases::C2,
                        11 | 23 | 47 | 59                       => AtkinCases::C3,
                        _                                       => AtkinCases::C4,
                    };
                    SieveOfAtkin::process_case_static(val.value(), integer, case, &mut test);
                }
                t.send(test).unwrap();
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }

        for _ in 0..9 {
            match rx.recv() {
                Ok(x)   => tests.push(x),
                _       => (),
            }
        }

        for i in tests {
            for j in 0..self.tests.len()-1 {
                let current = self.tests.get(j).unwrap().is_prime();
                self.tests.get_mut(j).unwrap().set_prime(current ^ i.get(j).unwrap().is_prime());
            }
        }

        // start sieving
        while !self.tests.is_empty() {
            let first = self.tests.first()
                .expect("Attempted to get item from tests vector while it was empty.")
                .clone();
            if first.is_prime() {
                self.results.push(first.value());
                let square = first.value() * first.value();
                let naturals = 1..;
                for i in naturals {
                    let n_square = i*square;
                    if n_square > self.tests.last()
                        .expect("Attempted to get item from tests vector while it was empty.")
                        .value() {
                        break;
                    }

                    match self.tests.binary_search(&TestInteger{ value: n_square, is_prime: true }) {
                        Ok(index)   => {
                            self.tests.get_mut(index)
                                    .expect("Attempted to get known index from tests vector. This shouldn't happen.")
                                    .set_prime(false);
                        },
                        Err(_)      => (),
                    }
                }
            }
            self.tests.remove(0);
        }
    }
}

