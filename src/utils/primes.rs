use std::iter::range_step;

//-------------------------------------------------------------------------------
// Primes module
//-------------------------------------------------------------------------------
// struct PrimeList {
//     primes: Vec<u64>,
//     last: u64
// }

// impl Iterator for PrimeList {
//     type Item = u64;

//     fn next(&mut self) -> Option<u64> {
//         let mut current = self.last + 1;

//         while !is_prime(current, &self.primes) {
//             current +=1;
//         }
//         self.primes.push(current);
//         self.last = current;
//         Some(current)
//     }
// }

// // bad name... to change.. do not export
// fn is_prime (n: u64, primes: &Vec<u64>) -> bool {
//     for p in primes.iter() {
//         if n % *p == 0 { return false; }
//     }
//     return true;
// }

// pub fn primes_iterator() -> PrimeList {
//     PrimeList{ primes: vec![], last: 1 }
// }

//-------------------------------------------------------------------------------
struct PrimeFactors {
    primes: EratostheneSieve,
    current: u64,
    primes_stack: Vec<u64>
    // last_prime: u64,
}

impl Iterator for PrimeFactors {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        match self.primes_stack.pop() {
            Some(p) => { 
                Some(p) 
            },
            None => {
                if self.current == 1 { return None; }
                let current = self.current;
                let p = self.primes.find(|n| current % *n == 0).expect("");
                self.current = self.current/p;
                while self.current % p == 0 {
                    self.current = self.current / p;
                    self.primes_stack.push(p);
                }

                Some(p)
            }
        }
        // if self.current % self.last_prime == 0 {
        //     self.current = self.current / self.last_prime;
        //     return Some(self.last_prime);
        // }
        // if self.current == 1 {
        //     return None;
        // }
        // for p in self.primes.iter() {
        //     if *p >= self.last_prime && self.current % *p == 0 {
        //         self.current = self.current / *p;
        //         self.last_prime = *p;
        //         return Some(*p);
        //     }
        // }
        // None
    }
}

fn find_next_prime() ->  u64 {
    5
}

pub fn factors_iterator(n: u64) -> PrimeFactors {
    PrimeFactors { 
        primes: eratosthene(n), 
        current: n, 
        primes_stack: vec![]
        // last_prime: n + 1 
    }
} 

static first_primes: Vec<u64> = primes_up_to(1000);

//-------------------------------------------------------------------------------
struct EratostheneSieve {
    last: u64,
    max: u64,
    sieve: Vec<bool>,
}

impl Iterator for EratostheneSieve {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        for i in (self.last + 1)..(self.max) {
            if self.sieve[i as usize] {
                for j in range_step(i + i, self.max, i) { self.sieve[j as usize] = false }
                self.last = i;
                return Some(i)
            }
        }
        None
    }
}

pub fn eratosthene(max: u64) -> EratostheneSieve {
    let mut sieve = Vec::new();
    for _ in (0..max) { sieve.push(true) }

    EratostheneSieve { last: 1, max: max, sieve: sieve }
}

pub fn primes_up_to(max: u64) -> Vec<u64> {
    eratosthene(max).collect()
}