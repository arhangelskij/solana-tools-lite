use solana_tools_lite::codec::{read_shortvec_len, write_shortvec_len};

fn roundtrip(value: usize, expected: &[u8]) {
    let mut buf = Vec::new();
    write_shortvec_len(value, &mut buf);
    assert_eq!(buf, expected);

    let (decoded, consumed) = read_shortvec_len(&buf).expect("decode");
    assert_eq!(decoded, value);
    assert_eq!(consumed, buf.len());
}

#[test]
fn shortvec_boundaries_roundtrip() {
    roundtrip(0, &[0x00]);
    roundtrip(0x7F, &[0x7F]);
    roundtrip(0x80, &[0x80, 0x01]);
    roundtrip(0x3FFF, &[0xFF, 0x7F]);
    roundtrip(0x4000, &[0x80, 0x80, 0x01]);
}

#[test]
fn shortvec_decode_empty_fails() {
    let err = read_shortvec_len(&[]).expect_err("empty should fail");
    let text = format!("{err}");
    assert!(text.contains("invalid short_vec length"));
}
