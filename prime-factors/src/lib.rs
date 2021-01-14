struct Divisor {
    divisor:u64,
    counter:u64,
}

impl Divisor {
    fn new(divisor: u64) -> Self {
        Self {divisor,counter:2u64}
    }
}

impl Iterator for Divisor {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {

        loop {if (self.divisor % self.counter == 0) 
            & (self.divisor != 1) {

            self.divisor /= self.counter;
            return Some(self.counter.clone());
        } else if self.divisor == 1 {
            return None;
        }
        self.counter += 1;
        }}}

pub fn factors(n: u64) -> Vec<u64> {
    Divisor::new(n).collect()
}
