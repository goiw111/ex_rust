pub fn primes(n: u32) -> Vec<u32> {
    let mut v: Vec<u32> = vec![2];
    let mut c = 3u32;
    while v.len() <= n as usize {
        for i in 2..=c {
            if (c % i == 0) ^ (c == i) {
                break;
            } else if c == i {
                v.push(i);
            }
        }
        c += 2;
    }
    v
}

pub fn nth(n: u32) -> u32 {

    (2..105_000).filter(|x| (2..=350).filter(|i| (x % i == 0) ^ (x == i)).count() == 0 )
	primes(n).into_iter()
        .nth(n as usize)
        .unwrap()
}
