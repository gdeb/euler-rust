#[allow(dead_code)]

use std::iter::AdditiveIterator;

use utils::fibonacci::Fibonacci;
use utils::palindromes::is_palindrome;
use utils::primes;
use std::num::Int;

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
            }
        }
    }
    max
}

fn ex5() -> u64 {
    let mut scm = 1;
    for i in 1..21 {
        scm = i / utils::gcd(i, scm) * scm;
    }
    scm
}

#[allow(unstable)]
fn ex6() -> u64 {
    let square_of_sum = (100*101/2).pow(2);
    let sum_of_squares = (1..101).map(|n| n*n).sum();
    square_of_sum - sum_of_squares
}

fn ex7() -> u64 {
    primes::primes_iterator().nth(10001 - 1).expect("")
}

//-------------------------------------------------------------------------------
#[allow(unstable)]
fn ex10() -> u64 {
    primes::primes_iterator()
        .take_while(|n| *n <2_000_000)
        .sum() 
}

fn ex15() -> u64 {
    utils::choose(40,20)
}

//-------------------------------------------------------------------------------
fn main() {
    println!("Euler Project Solver!");
    
    let solvers = vec!(
        (1, ex1 as fn() -> u64), 
        (2, ex2 as fn() -> u64),
        (3, ex3 as fn() -> u64),
        (4, ex4 as fn() -> u64),
        (5, ex5 as fn() -> u64),
        (6, ex6 as fn() -> u64),
        (7, ex7 as fn() -> u64),
        (10, ex10 as fn() -> u64),
        (15, ex15 as fn() -> u64),
    );


    for &(i, solver) in solvers.iter() {
        println!("Exercise {}: {}", i, (solver)());
    }

}


