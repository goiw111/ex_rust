struct Divisor {
    divisor:u32,
    ntype  :u32,
}

impl Divisor {
    fn new(divisor: u32) -> Self {
        let mut ntype = 0;
        let mut d = divisor.clone();
        while d > 0 {
            d /= 10;
            ntype += 1;
        }
        Self {divisor,ntype}
    }
}

impl Iterator for Divisor {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.divisor > 0 {
            let r = self.divisor % 10;
            self.divisor /= 10;
            Some(r.pow(self.ntype))
        } else {
            None
        }
    }
}


pub fn is_armstrong_number(num: u32) -> bool {
    Divisor::new(num).sum::<u32>() == num
}
