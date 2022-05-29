pub struct Rand {
    next: i64,
}

impl Rand {
    pub fn new() -> Self {
        Rand { next: 1 }
    }

    pub fn srand(seed: i64) -> Self {
        let mut n = Rand::new();
        n.next = seed;
        n
    }

    // CREDIT: http://www.open-std.org/jtc1/sc22/wg14/www/docs/n2310.pdf // page 256
    // RAND_MAX assumed to be 32767
    // there is some unresolved jank here. this behaves nothing like c rand but it does look like rand ints.
    pub fn rand(&mut self) -> i64 {
        self.next = (std::num::Wrapping(self.next) * std::num::Wrapping(1103515245) + std::num::Wrapping(12345)).0.abs();
        return ((self.next / 65536) % 32768) as i64;
    }
}
