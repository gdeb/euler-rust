//-------------------------------------------------------------------------------
// Fibonacci module
// Allow creaton of an iterator to iterate through fibonacci numbers
//-------------------------------------------------------------------------------
pub struct Fibonacci {
    current: u64,
    next: u64,
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let temp = self.current + self.next;
        self.current = self.next;
        self.next = temp;
        Some(self.current)
    }
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { current: 1, next: 1}
    }
}
