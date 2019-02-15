#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (short, long, cmp) = if first_list.len() == second_list.len() {
        if first_list.len() == 0 {
            return Comparison::Equal;
        }
        (first_list, second_list, Comparison::Equal)
    } else if first_list.len() < second_list.len() {
        (first_list, second_list, Comparison::Sublist)
    } else {
        (second_list, first_list, Comparison::Superlist)
    };
    for start in 0..long.len() {
        if long.len() - start < short.len() {
            return Comparison::Unequal;
        }
        if long
            .iter()
            .skip(start)
            .zip(short.iter())
            .map(|(e1, e2)| e1 == e2)
            .all(|eq| eq)
        {
            return cmp;
        }
    }
    return Comparison::Unequal;
}
