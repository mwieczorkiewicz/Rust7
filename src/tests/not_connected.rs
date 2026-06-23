use crate::{S7Client, S7Error, S7_AREA_DB, S7_WL_BYTE};

// All methods below use valid parameters to isolate the NotConnected guard.
// check_area and wordlen checks run before the connected check in read_area/write_area.

#[test]
fn read_area_not_connected() {
    let mut c = S7Client::new();
    let mut buf = [0u8; 4];
    assert!(matches!(
        c.read_area(S7_AREA_DB, 1, 0, S7_WL_BYTE, &mut buf),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn write_area_not_connected() {
    let mut c = S7Client::new();
    let buf = [0u8; 4];
    assert!(matches!(
        c.write_area(S7_AREA_DB, 1, 0, S7_WL_BYTE, &buf),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn read_db_not_connected() {
    let mut c = S7Client::new();
    let mut buf = [0u8; 4];
    assert!(matches!(
        c.read_db(1, 0, &mut buf),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn write_db_not_connected() {
    let mut c = S7Client::new();
    let buf = [0u8; 4];
    assert!(matches!(
        c.write_db(1, 0, &buf),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn read_bit_not_connected() {
    let mut c = S7Client::new();
    // bit_idx=0 is valid, so we reach the connected check inside read_area
    assert!(matches!(
        c.read_bit(S7_AREA_DB, 1, 0, 0),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn write_bit_not_connected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.write_bit(S7_AREA_DB, 1, 0, 0, true),
        Err(S7Error::NotConnected)
    ));
}

#[test]
fn read_area_invalid_wordlen_before_connected_check() {
    // wordlen validation happens before the connected check; expects InvalidFunParameter, not NotConnected
    let mut c = S7Client::new();
    let mut buf = [0u8; 4];
    assert!(matches!(
        c.read_area(S7_AREA_DB, 1, 0, 0x03, &mut buf),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn read_area_invalid_area_before_connected_check() {
    let mut c = S7Client::new();
    let mut buf = [0u8; 4];
    assert!(matches!(
        c.read_area(0x00, 1, 0, S7_WL_BYTE, &mut buf),
        Err(S7Error::InvalidFunParameter)
    ));
}
