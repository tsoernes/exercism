pub fn brackets_are_balanced(string: &str) -> bool {
    const L_BRACKETS: [char; 3] = ['[', '{', '('];
    const R_BRACKETS: [char; 3] = [']', '}', ')'];

    let str_brackets = string
        .chars()
        .filter(|c| L_BRACKETS.contains(c) || R_BRACKETS.contains(c));
    let mut stack = vec![];
    for brack in str_brackets {
        match L_BRACKETS.iter().position(|b| b == &brack) {
            Some(brack_ix) => stack.push(brack_ix),
            None => {
                // Encountered a right bracket in the string, so popped a left bracket
                // from the stack. If they are not the same kind of bracket,
                // the expression is not balanced, or if the stack is empty,
                // encountered a right bracket with an unmatched left bracket.
                match stack.pop() {
                    Some(brack_ix) if brack == R_BRACKETS[brack_ix] => {}
                    _ => return false,
                }
            }
        }
    }
    // If the stack is not empty, there's one or more left brackets with missing
    // right bracket(s).
    stack.is_empty()
}
