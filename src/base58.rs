// Useful reference: https://learnmeabitcoin.com/technical/base58

static B58_ALPHABET: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

#[derive(Debug, PartialEq, Eq)]
pub struct Base58Error;

impl std::fmt::Display for Base58Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unparseable base68 value")
    }
}

impl std::error::Error for Base58Error {}

#[allow(dead_code)]
pub fn encode(num: u64) -> String {
    // Reserve a byte vector of some minimal but usually sufficient amount
    let mut output: Vec<u8> = Vec::with_capacity(10);

    let mut num = num;
    while num > 0 {
        let idx = num % 58;
        output.push(B58_ALPHABET[idx as usize]);
        num = num / 58;
    }

    // Handle special case
    if output.is_empty() {
        output.push(b'1');
    }

    // Reverse to correct byte order
    output.reverse();

    // Convert to string
    String::from_utf8(output).expect("This should never fail")
}

#[allow(dead_code)]
pub fn decode(inp: &str) -> Result<u64, Base58Error> {
    inp.bytes().rev().enumerate().map(decode_factor).sum()
}

fn b58pos(byte: u8) -> Result<usize, Base58Error> {
    B58_ALPHABET
        .iter()
        .position(|d| *d == byte)
        .ok_or(Base58Error)
}

fn decode_factor((idx, byte): (usize, u8)) -> Result<u64, Base58Error> {
    let pos = b58pos(byte)?;
    let exp: u32 = idx.try_into().or(Err(Base58Error))?;
    let factor = pos * 58usize.pow(exp);
    Ok(factor as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let test_cases = [(0, "1"), (20, "M"), (9999, "3yQ")];

        for (inp, out) in test_cases {
            assert_eq!(encode(inp), out);
        }
    }

    #[test]
    fn test_decode() {
        let test_cases = [(0, "1"), (20, "M"), (9999, "3yQ")];
        for (out, inp) in test_cases {
            assert_eq!(decode(inp).unwrap(), out);
        }
    }

    #[test]
    fn test_invalid_decode() {
        assert_eq!(decode("0OIl"), Err(Base58Error));
    }
}
