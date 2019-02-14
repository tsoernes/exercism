use std::collections::HashSet;

pub fn brackets_are_balanced(string: &str) -> bool {
    let brackets: HashSet<char> = ['[', '{', '(', ']', '}', ')'].iter().cloned().collect();
    let str_brackets = string.chars().filter(|c| brackets.contains(c));
    let mut stack = Vec::with_capacity(string.len());
    for brack in str_brackets.into_iter() {
        match brack {
            '[' => stack.push(brack),
            '{' => stack.push(brack),
            '(' => stack.push(brack),
            // ']' => return stack.pop().map_or(false, |x| x == '['),
            ']' => {
                if !stack.pop().map_or(false, |x| x == '[') {
                    return false;
                }
            }
            '}' => {
                if !stack.pop().map_or(false, |x| x == '{') {
                    return false;
                }
            }
            ')' => {
                if !stack.pop().map_or(false, |x| x == '(') {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.is_empty()
}
