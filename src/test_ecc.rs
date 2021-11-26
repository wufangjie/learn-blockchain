#[test]
fn test_ecc() {
    for i in 1..1000 {
        if 20 * i % 23 == 1 {
            dbg!(i);
            break;
        }
    }
}
