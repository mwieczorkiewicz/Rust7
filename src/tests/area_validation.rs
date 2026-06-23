use crate::{S7Client, S7Error, S7_AREA_DB, S7_AREA_MK, S7_AREA_PA, S7_AREA_PE};

#[test]
fn process_inputs_valid() {
    let mut c = S7Client::new();
    assert!(c.check_area(S7_AREA_PE).is_ok());
}

#[test]
fn process_outputs_valid() {
    let mut c = S7Client::new();
    assert!(c.check_area(S7_AREA_PA).is_ok());
}

#[test]
fn merkers_valid() {
    let mut c = S7Client::new();
    assert!(c.check_area(S7_AREA_MK).is_ok());
}

#[test]
fn data_block_valid() {
    let mut c = S7Client::new();
    assert!(c.check_area(S7_AREA_DB).is_ok());
}

#[test]
fn zero_area_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.check_area(0x00),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn below_range_area_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.check_area(0x80),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn above_range_area_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.check_area(0x85),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn max_byte_area_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.check_area(0xFF),
        Err(S7Error::InvalidFunParameter)
    ));
}
