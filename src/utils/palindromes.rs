#[allow(dead_code)]


// -------------------------------------------------------------------------------
// Palindrome module
// -------------------------------------------------------------------------------

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

