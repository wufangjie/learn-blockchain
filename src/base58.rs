use crate::conv;
use crate::error::DecodeBase58Error;
use bitcoin_hashes::{hash160, sha256d, Hash};

const BASE58_CHARS: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn convert_base(lst: &[u8], b1: usize, b2: usize) -> Vec<u8> {
    if lst.len() == 0 {
        return vec![];
    }
    let mut left = vec![];
    let mut carry = 0usize;
    for item in lst.iter() {
        carry *= b1;
        carry += *item as usize;
        if !left.is_empty() || carry >= b2 {
            left.push((carry / b2) as u8);
        }
        carry %= b2;
    }

    let mut ret = convert_base(&left, b1, b2);
    ret.push(carry as u8);
    ret
}

fn encode_base58(lst: &[u8]) -> Vec<u8> {
    let n = lst.iter().take_while(|&&x| x == 0).count();
    let mut ret = vec![b'1'; n];
    ret.extend(
        convert_base(&lst[n..], 256, 58)
            .into_iter()
            .map(|idx| BASE58_CHARS[idx as usize])
            .collect::<Vec<u8>>(),
    );
    ret
}

fn decode_base58(lst: &[u8]) -> Result<Vec<u8>, DecodeBase58Error> {
    let mut lst_new = vec![];
    for (idx, x) in lst.iter().enumerate() {
        lst_new.push(match x {
            b'1'..=b'9' => x - b'1',
            b'A'..=b'H' => x - b'A' + 9,
            b'J'..=b'N' => x - b'A' + 8,
            b'P'..=b'Z' => x - b'A' + 7,
            b'a'..=b'k' => x - b'a' + 7 + 26,
            b'm'..=b'z' => x - b'a' + 6 + 26,
            _ => return Err(DecodeBase58Error::InvalidChar { c: *x as char, idx }),
        })
    }
    let n = lst.iter().take_while(|&&x| x == b'1').count();
    let mut ret = vec![0; n];
    ret.extend(convert_base(&lst_new[n..], 58, 256));
    Ok(ret)
}

pub fn encode_base58_check(prefix: &Vec<u8>, hex_str: &Vec<u8>) -> Vec<u8> {
    let mut to_hash: Vec<u8> = vec![];
    to_hash.extend(prefix);
    to_hash.extend(hash160::Hash::hash(&hex_str).into_inner());
    let check_sum = sha256d::Hash::hash(&to_hash);
    to_hash.extend(check_sum.into_iter().take(4));
    encode_base58(&to_hash)
}

pub fn decode_base58_check() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base58_encode() {
        // normal cases
        let f = |x| conv::bytes_to_string(&encode_base58(x));
        assert_eq!(f(b""), "");
        assert_eq!(f(b"abc"), "ZiCa");
        assert_eq!(f(b"1234598760"), "3mJr7AoUXx2Wqd");
        assert_eq!(
            f(b"abcdefghijklmnopqrstuvwxyz"),
            "3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f"
        );

        // prefix \0 cases
        assert_eq!(f(b"\0\0\0\0abc"), "1111ZiCa");
        assert_eq!(f(b"\0\0\0\0"), "1111");
    }

    #[test]
    fn test_base58_decode() {
        // invalid
        assert!(decode_base58(b"0").is_err());
        assert!(decode_base58(b"I").is_err());
        assert!(decode_base58(b"O").is_err());
        assert!(decode_base58(b"l").is_err());

        // normal cases
        let f = |x| conv::bytes_to_string(&decode_base58(x).unwrap());
        assert_eq!(f(b""), "");
        assert_eq!(f(b"ZiCa"), "abc");
        assert_eq!(
            f(b"3yxU3u1igY8WkgtjK92fbJQCd4BZiiT1v25f"),
            "abcdefghijklmnopqrstuvwxyz"
        );

        // prefix 1 cases
        assert_eq!(f(b"1111ZiCa"), "\0\0\0\0abc");
        assert_eq!(f(b"1111"), "\0\0\0\0");
    }

    #[test]
    fn test_base58_check() {
        assert_eq!(
            conv::bytes_to_string(&encode_base58_check(
                &vec![0u8],
                &conv::hex_to_bytes(
                    "0202a406624211f2abbdc68da3df929f938c3399dd79fac1b51b0e4ad1d26a47aa"
                )
                .unwrap()
            )),
            "1PRTTaJesdNovgne6Ehcdu1fpEdX7913CK"
        );
    }
}
