const FIRST: u8 = b'a';
const LAST: u8 = b'z';

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut chars_pushed = 0;
    plain
        .replace(".", "")
        .replace(",", "")
        .replace(" ", "")
        .to_lowercase()
        .chars()
        .filter(|x| x.is_ascii())
        .flat_map(|ch| {
            let mut res = vec![];
            if chars_pushed > 0 && chars_pushed % 5 == 0 {
                res.push(' ');
            }
            let mut ch_n = ch as u8;
            if ch.is_ascii_lowercase() {
                ch_n = FIRST + LAST - ch_n;
            }
            chars_pushed += 1;
            res.push(ch_n as char);
            res
        })
        .collect()
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .replace(" ", "")
        .chars()
        .map(|ch| {
            if ch.is_ascii_lowercase() {
                (FIRST + LAST - ch as u8) as char
            } else {
                ch
            }
        })
        .collect()
}
