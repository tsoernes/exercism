use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // Given: (where n=sum)
    // a < b < c
    // a + b + c = n
    // a^2 + b^2 = c^2
    // We have that:
    // a^2 + b^2 = (n - a - b)^2
    // a^2 + b^2 = n^2 - 2na - 2nb + a^2 + 2ab + b^2
    // 0 = n^2 - 2na - 2nb + 2ab
    // b (2n - 2a) = n^2 - 2na
    // b  = (n^2 - 2na) / (2n - 2a)
    let sum_pow2: u32 = sum.pow(2);
    let triplets = (1..sum / 3)
        .into_par_iter()
        .filter_map(|side_a| {
            let side_b = (sum_pow2 - 2 * sum * side_a) / (2 * sum - 2 * side_a);
            let side_c = sum - side_a - side_b;
            let is_valid_triangle = side_a.pow(2) + side_b.pow(2) == side_c.pow(2);
            if is_valid_triangle {
                Some([side_a, side_b, side_c])
            } else {
                None
            }
        })
        // Stop when 'a' becomes larger than or equal to 'b'
        // to avoid redundant triplet permutations
        .map(|sides| {
            if sides[0] < sides[1] {
                Some(sides)
            } else {
                None
            }
        })
        .while_some()
        .collect();

    triplets
}
