const FIRST: u8 = 'a' as u8;
const LAST: u8 = 'z' as u8;

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let plain = plain
        .replace(".", "")
        .replace(",", "")
        .replace(" ", "")
        .to_lowercase();
    let n_spaces = plain.len() % 5;
    let mut enc: Vec<u8> = Vec::with_capacity(plain.len() + n_spaces);
    let mut chars_pushed = 0;
    for ch in plain.chars() {
        if chars_pushed > 0 && chars_pushed % 5 == 0 {
            enc.push(' ' as u8);
        }
        let mut ch_n = ch as u8;
        if ch_n >= FIRST && ch_n <= LAST {
            ch_n = FIRST + LAST - ch_n;
        }
        if ch_n < 128 {
            chars_pushed += 1;
            enc.push(ch_n);
        }
    }
    String::from_utf8(enc).unwrap()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    let dec: Vec<u8> = cipher
        .replace(" ", "")
        .chars()
        .map(|ch| {
            let ch_n = ch as u8;
            if ch_n >= FIRST && ch_n <= LAST {
                FIRST + LAST - ch_n
            } else {
                ch_n
            }
        })
        .collect();
    String::from_utf8(dec).unwrap()
}
