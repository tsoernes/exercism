// It's called a "saddle point" because it is greater than or equal to every
// element in its row and less than or equal to every element in its
// column.
// At most 1 saddle point per row (the max of the row)
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let n_rows = input.len();
    if n_rows == 0 {
        return vec![];
    }
    let n_cols = input[0].len();
    if n_cols == 0 {
        return vec![];
    }
    println!("input:");
    for row in input.iter() {
        println!("{:?}", row);
    }

    let mut rows_maxes = Vec::with_capacity(n_rows);
    for (_, row) in input.iter().enumerate() {
        let max = row.iter().max().unwrap();
        let amaxes = indecies_of(row.iter(), &max);
        rows_maxes.push(amaxes);
    }
    println!("row maxes {:?}", rows_maxes);
    let mut cols_mins = Vec::with_capacity(n_cols);
    for i in 0..n_cols {
        let mut min = input[0][i];
        for j in 1..n_rows {
            if input[j][i] < min {
                min = input[j][i];
            }
        }
        let mut amins = vec![];
        for j in 0..n_rows {
            if input[j][i] == min {
                amins.push(j);
            }
        }
        cols_mins.push(amins);
    }
    println!("col mins {:?}", cols_mins);
    let mut saddles = vec![];
    for (row, row_maxes) in rows_maxes.iter().enumerate() {
        for row_max_col in row_maxes.iter() {
            for (col, col_mins) in cols_mins.get(*row_max_col).iter().enumerate() {
                for col_min_row in col_mins.iter() {
                    if (*col_min_row) == row {
                        saddles.push((row, *row_max_col));
                    }
                }
            }
        }
    }
    saddles
}

fn indecies_of<I: Iterator>(i: I, e: &I::Item) -> Vec<usize>
where
    I::Item: Eq,
{
    let mut ixs = vec![];
    for (i, x) in i.enumerate() {
        if &x == e {
            ixs.push(i)
        }
    }
    ixs
}
// what to do if multiple indexes has same value as argmax?
// fn argmax<I: Iterator>(i: I) -> Option<usize>
// where
//     I::Item: PartialOrd + Copy,
// {
//     i.enumerate()
//         .fold(None, |mb_imax, (j, x)| {
//             mb_imax.map_or(Some((j, x)), |(i, max)| {
//                 Some(if x > max { (j, x) } else { (i, max) })
//             })
//         })
//         .map(|(i, _)| i)
// }
// let argmax = vec.iter().fold(None,|m,&x| m.map_or(Some(x), |mv| Some(if x > mv {x} else {mv})));
// let argmax = vec.iter().fold(None,|m,&x| m.map_or(Some(x), |mv| std::cmp::partial_max(x, mv)));
