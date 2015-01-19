use std::cmp::Ordering::{Less, Equal, Greater};
use std::iter::MultiplicativeIterator;

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

#[allow(unstable)]
pub fn choose(n: u64, k: u64) -> u64 {
    let mut factors: Vec<u64> = vec![];

    for i in (n - k + 1)..(n + 1) { factors.push(i); }
    for i in 1..(k+1) { 
        let mut n = i;
        factors = factors.map_in_place(|u| {
            if n == 1 { return u; }
            let d = gcd(u, n);
            n = n/d;
            u/d
        });
    }
    factors.into_iter().product()
}

// fn find_divisor<'v>(d: u64, v: &'v Vec<u64>) -> (&'v u64, u64) {
//     let mut elem;
//     let mut c;
//     for i in v.iter() {
//         elem = i;
//         c = gcd(*i, d);
//         if c > 1 { break } 
//     }
//     (elem, c)
// }

