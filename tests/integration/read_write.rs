use rust7::S7_AREA_DB;

use super::common;

#[test]
fn read_db1_hello_s7() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    let mut buf = [0u8; 10];
    client.read_db(1, 0, &mut buf).expect("read_db failed");
    assert_eq!(&buf[..9], b"Hello, S7");
}

#[test]
fn write_db2_read_back() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let rest_port = container.get_host_port_ipv4(8080).expect("port 8080 not mapped");
    common::provision_db(rest_port, 2, 1024);
    let mut client = common::connect_client(s7_port);

    let written = [0x41u8, 0x42, 0x43, 0x44];
    client.write_db(2, 0, &written).expect("write_db failed");

    let mut buf = [0u8; 4];
    client.read_db(2, 0, &mut buf).expect("read_db failed");
    assert_eq!(buf, written);
}

#[test]
fn read_bit_db1() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    // exercises the full bit-read protocol path; value depends on DB1 init data
    let result = client.read_bit(S7_AREA_DB, 1, 0, 0);
    assert!(result.is_ok(), "read_bit returned error: {:?}", result);
}

#[test]
fn write_bit_db2_then_verify() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let rest_port = container.get_host_port_ipv4(8080).expect("port 8080 not mapped");
    common::provision_db(rest_port, 2, 1024);
    let mut client = common::connect_client(s7_port);

    client
        .write_bit(S7_AREA_DB, 2, 0, 0, true)
        .expect("write_bit failed");
    let val = client
        .read_bit(S7_AREA_DB, 2, 0, 0)
        .expect("read_bit failed");
    assert!(val);
}

#[test]
fn large_read_auto_chunks() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let rest_port = container.get_host_port_ipv4(8080).expect("port 8080 not mapped");
    common::provision_db(rest_port, 2, 1024);
    let mut client = common::connect_client(s7_port);

    let mut buf = vec![0u8; 1024];
    client.read_db(2, 0, &mut buf).expect("read_db failed");
    // 1024 bytes / 462 bytes per chunk (480-byte PDU minus 18-byte header) = 3 chunks minimum
    assert!(
        client.chunks >= 3,
        "expected ≥3 chunks for 1024-byte read, got {}",
        client.chunks
    );
}
