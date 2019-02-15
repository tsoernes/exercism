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
    let triplets = (1..n)
        .into_iter()
        .map(|a| {
            let t1 = n.pow(2) - 2 * n * a;
            let t2 = 2 * n - 2 * a;
            let b = t1 / t2;
            let c = n - a - b;
            [a, b, c]
        })
        // Stop when 'a' becomes larger or equal to 'b'
        // to avoid redundant triplet permutations
        .take_while(|trip| trip[0] < trip[1])
        .filter(|trip| trip[0].pow(2) + trip[1].pow(2) == trip[2].pow(2))
        .collect();
    triplets
}
