use bitcoin_hashes::hex::ToHex;
use bitcoin_hashes::{sha256, sha256d, Hash};
use learn_blockchain::conv;
use utils::Timer;

#[test]
fn test_hash_block() {
    // https://www.blockchain.com/btc/block/0000000000000002a7bbd25a417c0374cc55261021e8a9ca74442b01284f0569

    let header = conv::toggle_endian(vec![
        "00000002", //"02000000", // version
        "00000000000000027e7ba6fe7bad39faf3b5a83daed765f05f7d1b71a1632249", // pre tx hash
        "5e049f4030e0ab2debb92378f53c0a6e09548aea083f3ab25e1d94ea1155e29d",
        // merkle root: (even nodes (if not dup last node)) no weight huffman tree
        &format!("{:08x}", 1388185038u32), // timestamp
        "1903a30c", // difficulty, changed per 2016 (= 24 * 10 * 14, 2 weeks) blocks
        &format!("{:08x}", 4215469401u32), // nonce
    ]);

    assert_eq!(
        "0000000000000002a7bbd25a417c0374cc55261021e8a9ca74442b01284f0569",
        sha256d::Hash::hash(&header).to_hex()
    );
    dbg!(sha256d::Hash::hash(&header).as_inner());
    dbg!(sha256::Hash::hash(sha256::Hash::hash(&header).as_inner()));
}

#[test]
fn test_hash_transaction() {
    // 0100000001c997a5e56e104102fa209c6a852dd90660a20b2d9c352423edce25857fcd3704000000004847304402204e45e16932b8af514961a1d3a1a25fdf3f4f7732e9d624c6c61548ab5fb8cd410220181522ec8eca07de4860a4acdd12909d831cc56cbbac4622082221a8768d1d0901ffffffff0200ca9a3b00000000434104ae1a62fe09c5f51b13905f07f06b99a2f7159b2225f374cd378d71302fa28414e7aab37397f554a7df5f142c21c1b7303b8a0626f1baded5c72a704f7e6cd84cac00286bee0000000043410411db93e1dcdb8a016b49840f8c53bc1eb68a382e97b1482ecad7b148a6909a5cb2e0eaddfb84ccf9744464f82e160bfa9b8b64f9d4c03f999b8643f656b412a3ac00000000
    let to_hash = conv::hex_to_bytes([
	"01",
	"00000001",
	"186f9f998a5aa6f048e51dd8419a14d8a0f1a8a2836dd734d2804fe65fa35779",
	"00000000",
	"8b", // 139 bytes
	"483045022100884d142d86652a3f47ba4746ec719bbfbd040a570b1deccbb6498c75c4ae24cb02204b9f039ff08df09cbe9f6addac960298cad530a863ea8f53982c09db8f6e381301410484ecc0d46f1918b30928fa0e4ed99f16a0fb4fde0735e7ade8416ab9fe423cc5412336376789d172787ec3457eee41c04f4938de5cc17b4a10fa336a8d752adf", // scriptSig
	"ffffffff",
	"02",
	"60e31600", // 0.015 0x16e360 == 1500000
	"00000000",
	"19", // 25 bytes
	"76a914ab68025513c3dbd2f7b92a94e0581f5d50f654e788ac", // scriptPubKey
	"d0ef8000", // 0x80efd0 == 8450000, the last 00 seems to be exponent
	"00000000",
	"19", // 25 bytes
	"76a9147f9b1a7fb68d60c536c2fd8aeaa53a8f3cc025a888ac", // scriptPubKey
	"00000000"].join("")).unwrap();

    assert_eq!(
        "0627052b6f28912f2703066a912ea577f2ce4da4caa5a5fbd8a57286c345c2f2",
        sha256d::Hash::hash(&to_hash).to_hex()
    );

    // scriptSig, see P128
    // 0x30
    // 0x45 69 bytes
    // 0x02
    // 0x21 33 bytes R
    // 0x00...cb R
    // 0x02
    // 0x20 32 bytes S
    // 0x4b...13 S

    // scriptPubKey, see P282
    // 0x76 means OP_DUP
    // 0xa9 means OP_HASH160 (i.e. RIPEMD160(Sha256(x)))
    // 0x14 push next 20 bytes into stack
    // 0x88 means OP_EQUALVERIFY
    // 0xac means OP_CHECKSIG

    // Biggest Bitcoin Transactions:
    // https://www.blockchain.com/btc/tx/b36bced99cc459506ad2b3af6990920b12f6dc84f9c7ed0dd2c3703f94a4b692

    // https://www.oreilly.com/library/view/programming-bitcoin/9781492031482/ch04.html

    // python script to get hex tx data, Seems not work today
    // import requests
    // txid="ed70b8c66a4b064cfe992a097b3406fa81ff09641fe55a709e4266167ef47891"
    // url = 'https://blockchain.info/en/tx/' + txid + '?format=hex'
    // r = requests.get(url)
    // print(r.text)
}

#[test]
fn test_mining() {
    // https://www.blockchain.com/btc/block/0000000000000002a7bbd25a417c0374cc55261021e8a9ca74442b01284f0569

    let mut header = conv::toggle_endian(vec![
        "00000002", //"02000000", // version
        "00000000000000027e7ba6fe7bad39faf3b5a83daed765f05f7d1b71a1632249", // pre tx hash
        "5e049f4030e0ab2debb92378f53c0a6e09548aea083f3ab25e1d94ea1155e29d",
        // merkle root: (even nodes (if not dup last node)) no weight huffman tree
        &format!("{:08x}", 1388185038u32), // timestamp
        "1903a30c", // difficulty, changed per 2016 (= 24 * 10 * 14, 2 weeks) blocks
        &format!("{:08x}", 4215469401u32), // nonce
    ]);

    let mut dt = [0u8; 32]; // difficulty target
    dt[7] = 0x0c;
    dt[8] = 0xa3;
    dt[9] = 0x03;

    let mut timer = Timer::new();
    for i in 4215369401..=u32::MAX {
        // 4215469401u32 // nonce
        // my pc 100,000 ~ several second
        for j in 0..4 {
            let m = 8 * (3 - j);
            header[76 + 3 - j] = ((i & (0xff << m)) >> m) as u8;
        }

        let mut bytes = sha256d::Hash::hash(&header).into_inner();
        bytes.reverse();
        if bytes < dt {
            println!("found: {}", i);
            break;
        }
    }
    timer.stop();
}
