#[allow(dead_code)]

//-------------------------------------------------------------------------------
// Primes module
//-------------------------------------------------------------------------------
pub fn is_prime(n: u64) -> bool {
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }

    let mut p = 3;
    while p*p <= n {
        if n % p == 0 { return false; }
        p = p + 2;
    }
    return true;
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut result = vec![];
    let mut current = n;
    while current % 2 == 0 {
        current = current / 2;
        result.push(2);
    }
    let mut p = 3;
    while p*p <= current {
        while current % p == 0 {
            current = current / p;
            result.push(p);
        }
        p = p + 2;
    }
    if current > 1 { result.push(current); }
    result 
}

//-------------------------------------------------------------------------------
// struct EratostheneSieve {
//     last: u64,
//     max: u64,
//     sieve: Vec<bool>,
// }

// impl Iterator for EratostheneSieve {
//     type Item = u64;

//     fn next(&mut self) -> Option<u64> {
//         for i in (self.last + 1)..(self.max) {
//             if self.sieve[i as usize] {
//                 for j in range_step(i + i, self.max, i) { self.sieve[j as usize] = false }
//                 self.last = i;
//                 return Some(i)
//             }
//         }
//         None
//     }
// }

// pub fn eratosthene(max: u64) -> EratostheneSieve {
//     let mut sieve = Vec::new();
//     for _ in (0..max) { sieve.push(true) }

//     EratostheneSieve { last: 1, max: max, sieve: sieve }
// }

// pub fn primes_up_to(max: u64) -> Vec<u64> {
//     eratosthene(max).collect()
// }