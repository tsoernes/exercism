pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let n_rows = input.len();
    if n_rows == 0 {
        return vec![];
    }
    let n_cols = input[0].len();
    if n_cols == 0 {
        return vec![];
    }

    // For each row, a vector with the indecies of the max element in that row
    let mut rows_maxes: Vec<Vec<usize>> = Vec::with_capacity(n_rows);
    for row in input.iter() {
        let max = row.iter().max().unwrap();
        let amaxes = (0..n_cols).filter(|col| row[*col] == *max).collect();
        rows_maxes.push(amaxes);
    }

    // For each col, a vector with the indecies of the min element in that col
    let mut cols_mins: Vec<Vec<usize>> = Vec::with_capacity(n_cols);
    for col in 0..n_cols {
        let min = (0..n_rows).map(|row| input[row][col]).min().unwrap();
        let amins = (0..n_rows).filter(|row| input[*row][col] == min).collect();
        cols_mins.push(amins);
    }

    let mut saddles = vec![];
    for (row, row_maxes) in rows_maxes.iter().enumerate() {
        for row_max_col in row_maxes.iter() {
            for col_mins in cols_mins.get(*row_max_col).iter() {
                for col_min_row in col_mins.iter() {
                    if (*col_min_row) == row {
                        // The point is simultaneously the max of its
                        // row and the min of its column, i.e. a saddle point
                        saddles.push((row, *row_max_col));
                    }
                }
            }
        }
    }
    saddles
}

// /// Given an iterator and an element, return a Vec of indecies
// /// of that element as they are encountered in the iterator
// fn indecies_of<I>(i: I, e: &I::Item) -> Vec<usize>
// where
//     I: Iterator,
//     I::Item: Eq,
// {
//     let mut ixs = vec![];
//     for (i, x) in i.enumerate() {
//         if &x == e {
//             ixs.push(i)
//         }
//     }
//     ixs
// }
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
