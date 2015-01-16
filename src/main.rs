use std::iter::AdditiveIterator;

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
    utils::fibonacci::iterator()
        .filter(|f| *f % 2 == 0)
        .take_while(|n| *n <= 4000000)
        .sum()
}

//-------------------------------------------------------------------------------
fn ex3() -> u64 {
    //600_851_475_143
    utils::primes::factors_iterator(210)
        .max()
        .expect("n should have at least one divisor")
}


//-------------------------------------------------------------------------------
#[allow(unstable)]
fn ex10() -> u64 {
    utils::primes::eratosthene(2_000_000)
        .sum() 
}

//-------------------------------------------------------------------------------
fn main() {
    println!("Euler Project Solver!");
    
    let solvers = [
        &ex1 as &Fn<(), u64>, 
        &ex2 as &Fn<(), u64>, 
        &ex3 as &Fn<(), u64>,
        &ex10 as &Fn<(), u64>,
    ];

    for (i, solver) in solvers.iter().enumerate() {
        println!("Exercise {}: {}", i+1, (*solver)());
    }

    // for i in utils::primes::eratosthene_sieve(20) {
    //     println!("{}", i);
    // }
    // println!("Done {:?}", [false; 10]);


    println!("{:?}", utils::primes::primes_up_to(1000));

    for i in utils:: primes::factors_iterator(20) {
        println!("{}",i);
    }
}




