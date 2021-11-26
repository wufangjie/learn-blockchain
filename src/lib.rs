pub mod base58;
pub mod base64;
pub mod conv;
pub mod error;


// use bitcoin_hashes::Hash;
// use bitcoin_hashes::hex::FromHex;

// pub trait ToBigEndian {
//     fn to_be(&self) -> String;
// }

// impl<'a, T> ToBigEndian for T
// where
//     T: Hash + FromHex,
// {
//     fn to_be(&self) -> String {
// 	conv::bytes_to_hex(&self.as_inner().iter())
//     }
// }
