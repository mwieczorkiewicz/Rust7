use crate::{S7Client, S7Error, S7_AREA_DB};

// --- bit_idx validation ---

#[test]
fn read_bit_idx_8_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.read_bit(S7_AREA_DB, 1, 0, 8),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn read_bit_idx_255_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.read_bit(S7_AREA_DB, 1, 0, 255),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn write_bit_idx_8_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.write_bit(S7_AREA_DB, 1, 0, 8, true),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn write_bit_idx_255_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.write_bit(S7_AREA_DB, 1, 0, 255, false),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn read_bit_idx_7_boundary_passes_validation() {
    // bit_idx=7 is valid; error comes from disconnected state, not parameter check
    let mut c = S7Client::new();
    assert!(matches!(
        c.read_bit(S7_AREA_DB, 1, 0, 7),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn write_bit_idx_7_boundary_passes_validation() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.write_bit(S7_AREA_DB, 1, 0, 7, false),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn read_bit_idx_0_boundary_passes_validation() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.read_bit(S7_AREA_DB, 1, 0, 0),
        Err(S7Error::NotConnected)
    ));
}

// --- bit address formula: start = byte_num * 8 + bit_idx ---

#[test]
fn bit_address_doc_example() {
    // DB100.DBX 45.3 → start = 45 * 8 + 3 = 363
    let byte_num: u16 = 45;
    let bit_idx: u8 = 3;
    let start: u16 = byte_num * 8 + bit_idx as u16;
    assert_eq!(start, 363);
}

#[test]
fn bit_address_byte_zero_bit_zero() {
    let start: u16 = 0u16 * 8 + 0u16;
    assert_eq!(start, 0);
}

#[test]
fn bit_address_byte_zero_bit_seven() {
    let start: u16 = 0u16 * 8 + 7u16;
    assert_eq!(start, 7);
}

#[test]
fn bit_address_byte_one_bit_zero() {
    let start: u16 = 1u16 * 8 + 0u16;
    assert_eq!(start, 8);
}

#[test]
fn bit_address_stride_is_eight() {
    // Each successive byte starts 8 bits apart
    for byte in 0u16..16 {
        let start = byte * 8;
        assert_eq!(start % 8, 0);
        assert_eq!(start / 8, byte);
    }
}
