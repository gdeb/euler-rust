#[allow(dead_code)]

use std::iter::AdditiveIterator;

use utils::fibonacci::Fibonacci;
use utils::palindromes::is_palindrome;
use utils::primes;

mod utils;

#[allow(unstable)]
fn ex1() -> u64 {
    (0..1000)
        .filter(|n| *n % 3 == 0 || *n % 5 == 0)
        .sum()
}

//-------------------------------------------------------------------------------
#[allow(unstable)]
fn ex2() -> u64 {
    Fibonacci::new()
        .filter(|f| *f % 2 == 0)
        .take_while(|n| *n <= 4_000_000)
        .sum()
}

//-------------------------------------------------------------------------------
fn ex3() -> u64 {
    // Why do i have to use * below?
    *primes::factors(600_851_475_143)
        .iter()
        .max()
        .expect("n should have at least one divisor")  // is it idiomatic? seems awkward
}

// fn ex4() -> u64 {
//     let mut products = vec![];
//     for i in 100..1000 {
//         for j in 100..1000 { products.push( i * j) }
//     }
//     *products.iter().filter(|n| is_palindrome(**n)).max().expect("") // why **?
// }
fn ex4() -> u64 {
    let mut max = 0;
    for i in 100..1000 {
        for j in 100..1000 { 
            if i*j > max && is_palindrome(i*j) {
                max = i*j;
                // if i*j > max { max = i*j }
            }
        }
    }
    max
}

//-------------------------------------------------------------------------------
#[allow(unstable)]
// fn ex10() -> u64 {
//     primes::eratosthene(2_000_000)
//         .sum() 
// }

//-------------------------------------------------------------------------------
fn main() {
    println!("Euler Project Solver!");
    
    let solvers = vec!(
        ex1 as fn() -> u64, 
        ex2 as fn() -> u64, 
        ex3 as fn() -> u64, 
        ex4 as fn() -> u64, 
    );


    for (i, solver) in solvers.iter().enumerate() {
        println!("Exercise {}: {}", i+1, (solver)());
    }

}




