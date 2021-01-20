use std::collections::HashMap;

fn is_nucleotide(nucleotide: char) -> Result<char,char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T'   =>Ok(nucleotide),
        _                       =>Err(nucleotide),
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match is_nucleotide(nucleotide) {
        Ok(n)   =>{
            let mut cntr = 0usize;
            for c in dna.chars() {
                match is_nucleotide(c) {
                    Ok(x) if x == n =>cntr += 1,
                    Err(a)          =>return Err(a),
                    _               =>{},
                }
            }
            Ok(cntr)
        },
        Err(n)  =>Err(n),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let res = ['A','T','C','G'].iter().map(|x| (x,count(*x,dna)));
    let mut map = HashMap::new();
    for (c,i) in res {
        match i {
            Err(x)  =>  return Err(x),
            Ok(x)   =>  {map.insert(*c,x);},
        }
    }
    Ok(map)
}
