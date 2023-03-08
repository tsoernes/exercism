/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // Spaces are allowed in the input, but they should be stripped before checking.
    let code = code.replace(' ', "");
    // Strings of length 1 or less are not valid.
    if code.len() <= 1 {
        return false;
    }
    // Convert string to vector of ints
    let mbcnums: Option<Vec<i32>> =
        code.chars()
            .try_fold(vec![], |mut cnums, ch| match ch.to_digit(10) {
                Some(num) => {
                    cnums.push(num as i32);
                    Some(cnums)
                }
                None => None,
            });
    // All non-digit characters are disallowed.
    let Some(cnums) = mbcnums else { return false };

    let sum: i32 = cnums
        .iter()
        .rev()
        .enumerate()
        .map(|(ix, &cnum)| {
            // The first step of the Luhn algorithm is to double every second digit, starting from the right.
            // If doubling the number results in a number greater than 9 then subtract 9 from the product
            if ix > 0 && (ix + 1) % 2 == 0 {
                if cnum > 4 {
                    cnum * 2 - 9
                } else {
                    cnum * 2
                }
            } else {
                cnum
            }
        })
        // Then sum all digits
        .sum();
    (sum % 10) == 0
}
