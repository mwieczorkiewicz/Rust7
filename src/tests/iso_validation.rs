use crate::S7Error;

// check_iso_packet is a private free function in the client module.
// As a descendant module we can call it directly via the parent path.
use super::super::check_iso_packet;

fn valid_pkt(telegram_length: u16) -> [u8; 7] {
    [
        0x03, // TPKT RFC1006 ID
        0x00,
        (telegram_length >> 8) as u8,
        (telegram_length & 0xFF) as u8,
        0x02, // ISO length
        0xF0, // ISO PDU type
        0x80, // EOT
    ]
}

#[test]
fn valid_header_returns_remaining_bytes() {
    // telegram_length = 25, remaining = 25 - 7 = 18
    let mut pkt = valid_pkt(25);
    let result = check_iso_packet(480, &mut pkt);
    assert!(matches!(result, Ok(18)));
}

#[test]
fn valid_header_min_payload() {
    // telegram_length = 8, remaining = 1
    let mut pkt = valid_pkt(8);
    let result = check_iso_packet(480, &mut pkt);
    assert!(matches!(result, Ok(1)));
}

#[test]
fn valid_header_max_payload() {
    // telegram_length = 7 + pdu_length = 487, remaining = 480
    let mut pkt = valid_pkt(487);
    let result = check_iso_packet(480, &mut pkt);
    assert!(matches!(result, Ok(480)));
}

#[test]
fn wrong_tpkt_version_rejected() {
    let mut pkt = valid_pkt(25);
    pkt[0] = 0x04; // not ISO_ID (0x03)
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoInvalidHeader)
    ));
}

#[test]
fn wrong_iso_length_byte_rejected() {
    let mut pkt = valid_pkt(25);
    pkt[4] = 0x03; // not 0x02
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoInvalidHeader)
    ));
}

#[test]
fn wrong_pdu_type_rejected() {
    let mut pkt = valid_pkt(25);
    pkt[5] = 0xE0; // not 0xF0
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoInvalidHeader)
    ));
}

#[test]
fn eot_not_set_fragmented() {
    let mut pkt = valid_pkt(25);
    pkt[6] = 0x00; // not EOT (0x80)
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoFragmentedPacket)
    ));
}

#[test]
fn zero_payload_telegram_rejected() {
    // telegram_length = 7, remaining = 0
    let mut pkt = valid_pkt(7);
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoInvalidTelegram)
    ));
}

#[test]
fn telegram_shorter_than_header_rejected() {
    // telegram_length = 6 (less than TPKT_ISO_LEN=7)
    let mut pkt = valid_pkt(6);
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoInvalidTelegram)
    ));
}

#[test]
fn oversized_telegram_rejected() {
    // telegram_length = 7 + 481 = 488, pdu_length = 480 → remaining 481 > 480
    let mut pkt = valid_pkt(488);
    assert!(matches!(
        check_iso_packet(480, &mut pkt),
        Err(S7Error::IsoInvalidTelegram)
    ));
}
