pub fn brackets_are_balanced(string: &str) -> bool {
    let l_brackets: Vec<char> = vec!['[', '{', '('];
    let r_brackets: Vec<char> = vec![']', '}', ')'];

    let str_brackets: Vec<char> = string
        .chars()
        .filter(|c| l_brackets.contains(c) || r_brackets.contains(c))
        .collect();
    let mut stack = Vec::with_capacity(string.len());
    for brack in str_brackets.into_iter() {
        if l_brackets.contains(&brack) {
            stack.push(brack);
        } else if let Some(l_brack) = stack.pop() {
            // Encountered a right bracket in the string, so popped a left bracket
            // from the stack. If they are not the same kind of bracket,
            // the expression is not balanced.
            let l_brack_ix = r_brackets.iter().position(|b| b == &brack).unwrap();
            if l_brack != *l_brackets.get(l_brack_ix).unwrap() {
                return false;
            }
        } else {
            // Encountered a right bracket with an unmatched left bracket.
            return false;
        }
    }
    // If the stack is not empty, there's one or more left brackets with missing
    // right bracket(s).
    stack.is_empty()
}
