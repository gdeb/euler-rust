

// -------------------------------------------------------------------------------
// Palindrome module
// -------------------------------------------------------------------------------


// pub mod palindrome {
pub fn is_palindrome (mut n: u64) -> bool {
    let mut digits = vec![];

    loop {
        digits.push(n % 10);
        if n < 10 { break }
        n = n / 10;
    }
    let nbr_digits = digits.len();

    for i in (0..nbr_digits) {
        if digits[i] != digits[nbr_digits -1 - i] { return false; }
    }
    true
}

// }


// struct NumberIterator {

// }
// struct Sieve {

// }

// impl Iterator for Sieve {
//     type Item = u64;

//     fn next(&mut self) -> Option<u64> {
//         let p = self.sieve.next().expect("r");
//         self.sieve =self.sieve.filter(|n| n % p === 0);
//         Some(p)
//     }
// }






