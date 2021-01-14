pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {panic!("Square must be between 1 and 64")}
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    (2u128.pow(64) - 1u128) as u64  // Sn = a(r^n-1)/(r-1)
}
