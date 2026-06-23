// Podman users: export DOCKER_HOST=unix:///run/user/$(id -u)/podman/podman.sock
use std::io::{Read, Write};
use std::net::TcpStream;

use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::SyncRunner,
    Container, GenericImage, ImageExt,
};

use rust7::S7Client;

pub fn start_softplc() -> Container<GenericImage> {
    GenericImage::new("fbarresi/softplc", "latest-linux")
        .with_exposed_port(102.tcp())
        .with_exposed_port(8080.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Application started."))
        .with_env_var("DATA_PATH", "/demodata")
        .start()
        .expect("SoftPLC container failed to start")
}

pub fn provision_db(rest_port: u16, db_id: u16, size: u16) {
    let addr = format!("127.0.0.1:{rest_port}");
    let mut stream = TcpStream::connect(&addr).expect("REST port not reachable");
    let req = format!(
        "POST /api/DataBlocks?id={db_id}&size={size} HTTP/1.1\r\n\
         Host: 127.0.0.1:{rest_port}\r\n\
         accept: */*\r\n\
         Content-Length: 0\r\n\r\n"
    );
    stream.write_all(req.as_bytes()).unwrap();
    let mut resp = [0u8; 512];
    let _ = stream.read(&mut resp);
}

pub fn connect_client(s7_port: u16) -> S7Client {
    let mut client = S7Client::new();
    client
        .set_connection_port(s7_port)
        .expect("set_connection_port failed");
    client
        .connect_s71200_1500("127.0.0.1")
        .expect("connect_s71200_1500 failed");
    client
}
