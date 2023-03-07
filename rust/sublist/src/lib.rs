use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => return Comparison::Equal,
        (0, _) => return Comparison::Sublist,
        (_, 0) => return Comparison::Superlist,
        (_, _) => {}
    }
    let (short, long, cmp) = match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => (first_list, second_list, Comparison::Equal),
        Ordering::Less => (first_list, second_list, Comparison::Sublist),
        Ordering::Greater => (second_list, first_list, Comparison::Superlist),
    };

    if long.windows(short.len()).any(|window| window == short) {
        cmp
    } else {
        Comparison::Unequal
    }
}
