// Tests for hi_part!, lo_part!, make_u16! macros.
// These macro_rules! macros are in textual scope from client.rs (the declaring module).

#[test]
fn hi_part_typical() {
    assert_eq!(hi_part!(0x1234u16), 0x12u8);
}

#[test]
fn hi_part_zero() {
    assert_eq!(hi_part!(0x0000u16), 0x00u8);
}

#[test]
fn hi_part_max() {
    assert_eq!(hi_part!(0xFFFFu16), 0xFFu8);
}

#[test]
fn hi_part_low_byte_only() {
    assert_eq!(hi_part!(0x00FFu16), 0x00u8);
}

#[test]
fn lo_part_typical() {
    assert_eq!(lo_part!(0x1234u16), 0x34u8);
}

#[test]
fn lo_part_zero() {
    assert_eq!(lo_part!(0x0000u16), 0x00u8);
}

#[test]
fn lo_part_max() {
    assert_eq!(lo_part!(0xFFFFu16), 0xFFu8);
}

#[test]
fn lo_part_high_byte_only() {
    assert_eq!(lo_part!(0xFF00u16), 0x00u8);
}

#[test]
fn make_u16_typical() {
    assert_eq!(make_u16!(0x12u8, 0x34u8), 0x1234u16);
}

#[test]
fn make_u16_zero() {
    assert_eq!(make_u16!(0x00u8, 0x00u8), 0x0000u16);
}

#[test]
fn make_u16_max() {
    assert_eq!(make_u16!(0xFFu8, 0xFFu8), 0xFFFFu16);
}

#[test]
fn make_u16_hi_only() {
    assert_eq!(make_u16!(0x01u8, 0x00u8), 0x0100u16);
}

#[test]
fn roundtrip_hi_lo() {
    let v: u16 = 0xAB89;
    assert_eq!(make_u16!(hi_part!(v), lo_part!(v)), v);
}

#[test]
fn roundtrip_pdu_len_req() {
    // PDU_LEN_REQ = 480 = 0x01E0
    let pdu: u16 = 480;
    assert_eq!(hi_part!(pdu), 0x01);
    assert_eq!(lo_part!(pdu), 0xE0);
    assert_eq!(make_u16!(hi_part!(pdu), lo_part!(pdu)), pdu);
}
