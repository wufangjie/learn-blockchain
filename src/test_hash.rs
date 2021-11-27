use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{sha256, sha256d, Hash};
use learn_blockchain::conv;

// NOTE: bitcoin_hashes's fmt, to_hex are all little-endian

#[test]
fn test_sha256() {
    assert_eq!(
        sha256::Hash::hash(b"hello world").to_hex(),
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );

    assert_eq!(
        sha256::Hash::hash(sha256::Hash::hash(b"hello world").as_inner()).to_hex(),
        "bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423"
    ); // twice hash, made the endian different?

    assert_eq!(
        sha256d::Hash::hash(b"hello world").to_hex(),
        "2344b7a9b50f3cc2761a40722c05361f73119f4d5d6cc129da369e0db8d462bc"
    );
}

// hash160(s) === ripemd160(sha256(s))
