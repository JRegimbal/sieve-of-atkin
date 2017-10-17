extern crate std;
use std::cmp::Ordering;

#[derive(Copy)]
#[derive(Eq)]
pub struct TestInteger {
    pub value: u64,
    pub is_prime: bool
}

impl Clone for TestInteger {
    fn clone(&self) -> TestInteger { *self }
}

impl Ord for TestInteger {
    fn cmp(&self, other: &TestInteger) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for TestInteger {
    fn partial_cmp(&self, other: &TestInteger) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TestInteger {
    fn eq(&self, other: &TestInteger) -> bool {
        (self.value == other.value) && (self.is_prime == other.is_prime)
    }
}

impl TestInteger {
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

