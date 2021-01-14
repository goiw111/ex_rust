use itertools::Itertools;
        
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors.iter().filter(|x| x != &&0u32)
        .flat_map(|u| (1u32..limit).filter(move |i| i % u == 0))
        .unique().sum()
}
