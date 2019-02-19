/// What should the type of _function be?
pub fn map<A, B, F: FnMut(A) -> B>(input: Vec<A>, mut function: F) -> Vec<B> {
    let mut results = Vec::with_capacity(input.len());
    for elem in input {
        results.push(function(elem));
    }
    results
}
