use std::cmp::Ordering::{Less, Equal, Greater};

pub mod fibonacci;
pub mod primes;
pub mod palindromes; 

pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 { return b; }
    if b == 0 { return a; }
    match a.cmp(&b) {
        Less => gcd(a, b % a),
        Equal => a,
        Greater => gcd(a % b, b),
    }
}