pub struct Triangle(f64,f64,f64);

impl Triangle {
    pub fn build<T: Into<f64> + Copy>(sides: [T; 3]) -> Option<Triangle>
    {
        let sides = [sides[0].into(),sides[1].into(),sides[2].into()];
        if sides[0].max(sides[1]).max(sides[2]) < sides.iter().sum::<f64>() / 2f64 {
            Some(Triangle(sides[0],sides[1],sides[2]))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        [self.0,self.1,self.2,self.0].windows(2).all(|x| x[0] == x[1])
    }

    pub fn is_scalene(&self) -> bool {
        [self.0,self.1,self.2,self.0].windows(2).all(|x| x[0] != x[1])
    }

    pub fn is_isosceles(&self) -> bool {
        [self.0,self.1,self.2,self.0].windows(2).any(|x| x[0] == x[1])
    }
}
