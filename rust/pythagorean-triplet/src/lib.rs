use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    // Given:
    // a < b < c
    // a + b + c = n
    // a^2 + b^2 = c^2
    // We have that:
    // a^2 + b^2 = (n - a - b)^2
    // a^2 + b^2 = n^2 - 2na - 2nb + a^2 + 2ab + b^2
    // 0 = n^2 - 2na - 2nb + 2ab
    // b (2n - 2a) = n^2 - 2na
    // b  = (n^2 - 2na) / (2n - 2a)
    let t0: u32 = n.pow(2);
    let triplets = (1..n / 3)
        .into_par_iter()
        .filter_map(|a| {
            let t1 = t0 - 2 * n * a;
            let t2 = 2 * n - 2 * a;
            let b = t1 / t2;
            let c = n - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                Some([a, b, c])
            } else {
                None
            }
        })
        // Stop when 'a' becomes larger than or equal to 'b'
        // to avoid redundant triplet permutations
        .map(|trip| if trip[0] < trip[1] { Some(trip) } else { None })
        .while_some()
        .collect();

    triplets
}
