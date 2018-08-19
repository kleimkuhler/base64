use std::char;

fn to_char(num: u8) -> char {
    let output = match num {
        0 ... 25 => b'A' + num,
        26 ... 51 => b'a' + num - 26,
        52 ... 61 => b'0' + num - 52,
        62 => b'+',
        63 => b'/',
        _ => panic!("num: {} out of base64 bounds!", num)
    };

    output as char
}

fn to_num(c: u8) -> u32 {
    let output = match c {
        b'A' ... b'Z' => c - b'A',
        b'a' ... b'z' => c - b'a' + 26,
        b'0' ... b'9' => c - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        b'=' => 0,
        _ => panic!("c: {} not a base64 character", c)
    };

    output as u32
}

pub fn encode(input: &str) -> String {
    let mut output = String::new();

    for bytes in input.as_bytes().chunks(3) {
        let mut buf = 0;
        
        buf += (bytes.get(0).cloned().unwrap_or(0) as u32) << 8*2;
        buf += (bytes.get(1).cloned().unwrap_or(0) as u32) << 8;
        buf += bytes.get(2).cloned().unwrap_or(0) as u32;

        for i in 0..4 {
            if bytes.len() >= i {
                let off = (3 - i) * 6;
                let val = (buf >> off & 0x3F) as u8;
                output.push(to_char(val))
            } else {
                output.push('=');
            }
        }
    }

    output
}

pub fn decode(input: &str) -> String {
    let mut output = String::new();

    for bytes in input.as_bytes().chunks(4) {
        let mut buf = 0;

        buf += (to_num(bytes[0]) as u32) << 6*3;
        buf += (to_num(bytes[1]) as u32) << 6*2;
        buf += (to_num(bytes[2]) as u32) << 6;
        buf += to_num(bytes[3]) as u32;

        for i in 1..4 {
            if bytes[i] != b'=' {
                let off = (3 - i) * 8;
                let val = char::from_u32(buf >> off & 0xFF).unwrap();
                output.push(val);
            }
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char() {
        assert_eq!(to_char(0), 'A');
        assert_eq!(to_char(26), 'a');
        assert_eq!(to_char(62), '+');
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode("Foo"), "Rm9v");
        assert_eq!(encode("Fo"), "Rm8=");
        assert_eq!(encode("F"), "Rg==");
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode("Rm9v"), "Foo");
        assert_eq!(decode("Rm8="), "Fo");
        assert_eq!(decode("Rg=="), "F");
    }
}
