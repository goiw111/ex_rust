pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound)
        .filter(|&x| (x % 2 != 0 || x == 2) && (x % 3 != 0 || x == 3) && (x % 5 != 0 || x == 5) && (x % 7 != 0 || x == 7) && (x % 11 != 0 || x == 11))
        .collect()
}
