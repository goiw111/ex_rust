pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut v = Vec::new();
        for l in 0..self.0 {
            let mut c = Vec::new();
            for i in 0..=l {
                if i == 0 || l == 0 {
                    c.push(1u32);
                } else {
                    c.push(c[(i-1) as usize] * (l - i + 1)/i);
                }
            }
            v.push(c);
        }
        v
    }
}
