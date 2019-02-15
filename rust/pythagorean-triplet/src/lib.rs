use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    // a < b < c
    for b in 1..n {
        let nb = n - b;
        for a in 1..b.min(nb) {
            let c = nb - a;
            if a * a + b * b == c * c {
                triplets.insert([a, b, c]);
            }
        }
    }
    triplets
}
