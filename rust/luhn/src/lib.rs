/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");
    if code.len() <= 1 {
        return false;
    }
    let mut cnums: Vec<u32> = Vec::with_capacity(code.len());
    for ch in code.chars() {
        let num = ch as i32 - 48;
        if num < 0 || num > 9 {
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
    let valid = (sum % 10) == 0;
    println!("{:?}, {:?}, {}, {}", code, cnums, sum, valid);
    valid
}
