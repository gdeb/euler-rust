#[allow(dead_code)]


// -------------------------------------------------------------------------------
// Palindrome module
// -------------------------------------------------------------------------------

pub fn is_palindrome (mut n: u64, base: u64) -> bool {
    let mut digits = vec![];

    loop {
        digits.push(n % base);
        if n < base { break }
        n = n / base;
    }
    let nbr_digits = digits.len();

    for i in (0..nbr_digits) {
        if digits[i] != digits[nbr_digits -1 - i] { return false; }
    }
    true
}

