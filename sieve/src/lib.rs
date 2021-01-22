pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut v = (2..=upper_bound).collect::<Vec<u64>>();
    let mut i = 0;
    let sqrt  = (upper_bound as f64).sqrt() as u64;
    while v[i] <= sqrt {
        let iv = v[i];
        v.retain(|&x| x % iv != 0 || x == iv );
        i += 1;
    }
    v
}
