/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }
    let mut cnums: Vec<u32> = Vec::with_capacity(code.len());
    for ch in code.chars() {
        let num = ch as i32 - 48; // '0' -> 0, '1' -> 1, etc..
        if num < 0 || num > 9 {
            // character is not a digit, hence code is invalid
            return false;
        }
        cnums.push(num as u32);
    }

    for cnum in cnums.iter_mut().rev().skip(1).step_by(2) {
        *cnum *= 2;
        if *cnum > 9 {
            *cnum -= 9;
        }
    }
    let sum: u32 = cnums.iter().sum();
    (sum % 10) == 0
}
