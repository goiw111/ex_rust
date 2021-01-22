#[derive(Debug, PartialEq)]
pub struct Dna(Vec<char>);

#[derive(Debug, PartialEq)]
pub struct Rna(Vec<char>);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let mut v = Vec::new();
        for (i,d) in dna.chars().enumerate() {
            match d {
                'G'|'C'|'T'|'A' =>v.push(d),
                _               =>return Err(i),
            }
        }
        Ok(Dna(v))
    }

    pub fn into_rna(self) -> Rna {
        Rna(self.0.into_iter().map(|x| match x {
            'A' =>'U',
            'G' =>'C',
            'C' =>'G',
            _   =>'A',
        }).collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let mut v = Vec::new();
        for (i,d) in rna.chars().enumerate() {
            match d {
                'C'|'G'|'A'|'U' =>v.push(d),
                _               =>return Err(i),
            }
        }
        Ok(Rna(v))
    }
}
