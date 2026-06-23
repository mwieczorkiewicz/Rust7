use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::OnceLock;

use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::SyncRunner,
    Container, GenericImage, ImageExt,
};

use rust7::S7Client;

/// Ensures DOCKER_HOST is set to a reachable socket before testcontainers initialises
/// its bollard client. When DOCKER_HOST is absent, bollard falls back to reading the
/// active Docker context; context names like "desktop-linux" are not valid URLs and
/// cause URLParseError. We probe well-known socket paths in priority order and set
/// DOCKER_HOST to the first one that exists.
///
/// Candidates (in order):
///   1. $HOME/.docker/run/docker.sock  — Docker Desktop (macOS / Linux)
///   2. /var/run/docker.sock           — standard Linux daemon / symlink
///   3. /run/docker.sock               — alternative Linux path
///   4. /run/user/<uid>/podman/podman.sock — Podman rootless (derived from $HOME path)
fn ensure_docker_host() {
    static ONCE: OnceLock<()> = OnceLock::new();
    ONCE.get_or_init(|| {
        let current = std::env::var("DOCKER_HOST").unwrap_or_default();
        // Only trust DOCKER_HOST if it looks like a real URL; an empty string or a
        // Docker context name (e.g. "desktop-linux") would cause bollard to fail with
        // URLParseError: RelativeUrlWithoutBase.
        if current.starts_with("unix://")
            || current.starts_with("tcp://")
            || current.starts_with("npipe://")
        {
            return;
        }
        let home = std::env::var("HOME").unwrap_or_default();
        // Derive a likely Podman rootless socket from the numeric UID embedded in HOME
        // e.g. /Users/1000 or /home/1000 — not reliable, so it's last in the list.
        let uid_socket = std::fs::read_to_string("/proc/self/loginuid")
            .ok()
            .map(|uid| format!("/run/user/{}/podman/podman.sock", uid.trim()))
            .unwrap_or_default();

        let candidates: &[&str] = &[
            &format!("{home}/.docker/run/docker.sock"),
            "/var/run/docker.sock",
            "/run/docker.sock",
            &uid_socket,
        ];
        for path in candidates {
            if !path.is_empty() && std::path::Path::new(path).exists() {
                std::env::set_var("DOCKER_HOST", format!("unix://{path}"));
                return;
            }
        }
    });
}

pub fn start_softplc() -> Container<GenericImage> {
    ensure_docker_host();
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
