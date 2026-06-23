use crate::{S7Client, S7Error, CT_PG, CT_OP, CT_S7};

// --- set_connection_type ---

#[test]
fn set_conn_type_pg_ok() {
    let mut c = S7Client::new();
    assert!(c.set_connection_type(CT_PG).is_ok());
}

#[test]
fn set_conn_type_op_ok() {
    let mut c = S7Client::new();
    assert!(c.set_connection_type(CT_OP).is_ok());
}

#[test]
fn set_conn_type_s7_ok() {
    let mut c = S7Client::new();
    assert!(c.set_connection_type(CT_S7).is_ok());
}

#[test]
fn set_conn_type_zero_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.set_connection_type(0),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn set_conn_type_out_of_range_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.set_connection_type(4),
        Err(S7Error::InvalidFunParameter)
    ));
}

// --- set_timeout ---

#[test]
fn set_timeout_all_positive_ok() {
    let mut c = S7Client::new();
    assert!(c.set_timeout(3000, 1000, 500).is_ok());
}

#[test]
fn set_timeout_connect_zero_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.set_timeout(0, 1000, 500),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn set_timeout_read_zero_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.set_timeout(3000, 0, 500),
        Err(S7Error::InvalidFunParameter)
    ));
}

#[test]
fn set_timeout_write_zero_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.set_timeout(3000, 1000, 0),
        Err(S7Error::InvalidFunParameter)
    ));
}

// --- set_connection_port ---

#[test]
fn set_port_102_ok() {
    let mut c = S7Client::new();
    assert!(c.set_connection_port(102).is_ok());
}

#[test]
fn set_port_max_ok() {
    let mut c = S7Client::new();
    assert!(c.set_connection_port(u16::MAX).is_ok());
}

#[test]
fn set_port_zero_rejected() {
    let mut c = S7Client::new();
    assert!(matches!(
        c.set_connection_port(0),
        Err(S7Error::InvalidFunParameter)
    ));
}

// --- disconnect ---

#[test]
fn disconnect_on_new_client_is_safe() {
    let mut c = S7Client::new();
    c.disconnect(); // must not panic
    assert!(!c.connected);
}

#[test]
fn disconnect_idempotent() {
    let mut c = S7Client::new();
    c.disconnect();
    c.disconnect(); // second call must not panic either
    assert!(!c.connected);
}
