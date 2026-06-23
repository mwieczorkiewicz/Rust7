use super::common;

#[test]
fn connect_succeeds() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let client = common::connect_client(s7_port);
    assert!(client.connected);
}

#[test]
fn pdu_length_negotiated() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let client = common::connect_client(s7_port);
    assert!(client.pdu_length > 0);
}

#[test]
fn disconnect_clears_flag() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);
    client.disconnect();
    assert!(!client.connected);
}

#[test]
fn reconnect_after_disconnect() {
    let container = common::start_softplc();
    let s7_port = container.get_host_port_ipv4(102).expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);
    client.disconnect();
    client
        .connect_s71200_1500("127.0.0.1")
        .expect("reconnect failed");
    assert!(client.connected);
}
