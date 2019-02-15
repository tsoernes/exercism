use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    for b in 1..n {
        for a in 1..b.min(n - b) {
            let c = n - a - b;
            if a * a + b * b == c * c {
                triplets.insert([a, b, c]);
            }
        }
    }
    triplets
}
