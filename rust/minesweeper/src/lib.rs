use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let n_rows = minefield.len();
    if n_rows == 0 {
        return vec![];
    }
    let n_cols = minefield[0].len();
    let mut out_rows: Vec<String> = vec![];
    for (row_ix, row) in minefield.iter().enumerate() {
        let mut out_row = "".to_owned();
        for (col_ix, ch) in row.as_bytes().iter().enumerate() {
            if ch == &b' ' {
                // an empty square, to be replaced with the count of horizontal, vertical, and diagonal mines
                let mut count = 0u32;
                // top left
                if (row_ix > 0)
                    && (col_ix > 0)
                    && minefield[row_ix - 1].as_bytes()[col_ix - 1] == b'*'
                {
                    count += 1;
                }
                // top middle
                if (row_ix > 0) && minefield[row_ix - 1].as_bytes()[col_ix] == b'*' {
                    count += 1;
                }
                // top right
                if (row_ix > 0)
                    && (col_ix + 1 < n_cols)
                    && minefield[row_ix - 1].as_bytes()[col_ix + 1] == b'*'
                {
                    count += 1;
                }
                // left
                if (col_ix > 0) && minefield[row_ix].as_bytes()[col_ix - 1] == b'*' {
                    count += 1;
                }
                // right
                if (col_ix + 1 < n_cols) && minefield[row_ix].as_bytes()[col_ix + 1] == b'*' {
                    count += 1;
                }
                // bottom left
                if (row_ix + 1 < n_rows)
                    && (col_ix > 0)
                    && minefield[row_ix + 1].as_bytes()[col_ix - 1] == b'*'
                {
                    count += 1;
                }
                // bottom middle
                if (row_ix + 1 < n_rows) && minefield[row_ix + 1].as_bytes()[col_ix] == b'*' {
                    count += 1;
                }
                // bottom right
                if (row_ix + 1 < n_rows)
                    && (col_ix + 1 < n_cols)
                    && minefield[row_ix + 1].as_bytes()[col_ix + 1] == b'*'
                {
                    count += 1;
                }
                if count > 0 {
                    let count_dig = char::from_digit(count, 10).unwrap();
                    out_row.push(count_dig);
                } else {
                    out_row.push(' ');
                }
            } else {
                // mines are represented as-is
                out_row.push('*');
            }
            println!(
                "row_ix: {}, row: {}, col_ix: {}, char: {}",
                row_ix, row, col_ix, ch
            )
        }
        out_rows.push(out_row);
    }
    out_rows
}
