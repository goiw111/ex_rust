use std::str;

//the impossible no regex mission lol
fn matcher(strn: &str) -> Vec<&str> {
    let s   = strn.as_bytes();
    let mut svec= Vec::new();
    let mut it  = 0;
        for i in 1..s.len() {
            match (&s[it..=i],&s[i+1..]) {
                (l,r)   =>match (l.last(),r.first()) {
                    (Some(ll),Some(rf)) =>
                        if ll.is_ascii_graphic() && rf.is_ascii_whitespace() 
                        {svec.push(str::from_utf8(l).unwrap());it += l.len();}
                        else if ll.is_ascii_lowercase() && rf.is_ascii_uppercase()
                        {svec.push(str::from_utf8(l).unwrap());it += l.len();}
                        else if ll.is_ascii_alphabetic() && *rf == b'-'
                        {svec.push(str::from_utf8(l).unwrap());it += l.len();}
                        else {},
                    _   =>{},},
            }
        }
        svec.push(str::from_utf8(&s[it..]).unwrap());
        println!("{:?}",svec);
        svec
}
pub fn abbreviate(phrase: &str) -> String {
    matcher(phrase).into_iter()
        .map(|c| c.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_ascii_whitespace()))
        .map(str::chars)
        .map(|mut x| x.next())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .filter(|x| x.is_ascii_alphabetic())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}
