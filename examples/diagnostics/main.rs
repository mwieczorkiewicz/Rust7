//! Diagnostic buffer example — demonstrates SZL (System Status List) reads.
//!
//! Usage:
//!   cargo run [PLC_IP]
//!
//! PLC_IP defaults to 127.0.0.1. Requires a connected S7-1200 / S7-1500 PLC
//! or the fbarresi/softplc container (see examples/docker/).
//!
//! Note: fbarresi/softplc does not implement ROSCTR 0x07 (Userdata) / SZL.
//! Against a real PLC all three operations below succeed.
//!
//! Sample output (S7-1516-3 PN/DP, abbreviated):
//!
//! ```text
//! rust7 — Diagnostic Buffer Example
//! ------------------------------------------------------------
//! Connecting to PLC at 192.168.0.100 ...
//! Connected
//! PDU negotiated : 480 bytes
//! Job time (ms)  : 2.341
//! ------------------------------------------------------------
//! Reading CPU info (SZL 0x001C) ...
//! Success!
//! Job time (ms)  : 1.823
//! Chunks         : 1
//! Module type    : CPU 1516-3 PN/DP
//! Module name    : PLC_1
//! AS name        : MY_S7_STATION
//! Serial number  : S C-E6T37943020
//! Copyright      : Original Siemens Equipment
//! ------------------------------------------------------------
//! Reading raw SZL 0x0011 (CPU identification) ...
//! Success!
//! Job time (ms)  : 1.102
//! Chunks         : 1
//! SZL header     : length_dr=28 n_dr=3
//! Payload (84 bytes):
//!   0000: 00 11 00 01 00 06 43 50 55 20 31 35 31 36 2D 33
//!   0010: 20 50 4E 2F 44 50 00 00 00 00 00 00 36 45 53 37
//!   0020: 2D 33 00 00 00 00 00 00 00 00 00 00 00 00 00 00
//!   0030: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
//!   0040: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
//!   0050: 00 00 00 00
//! ------------------------------------------------------------
//! Reading diagnostic buffer (SZL 0x00A0) ...
//! Success!
//! Job time (ms)  : 3.471
//! Chunks         : 1
//! Entries        : 4
//!
//!   [  0] event=0x4302  ts=2024-03-15 10:32:47.000  info=[00, 00, 00, 00]
//!          class=Mode transitions  name=Mode transition from STARTUP to RUN
//!   [  1] event=0x4301  ts=2024-03-15 10:32:46.000  info=[00, 00, 00, 00]
//!          class=Mode transitions  name=Mode transition from STOP to STARTUP
//!   [  2] event=0x4309  ts=2024-03-15 10:32:45.000  info=[00, 00, 00, 00]
//!          class=Mode transitions  name=Memory reset started automatically (power on not backed up)
//!   [  3] event=0x3501  ts=2024-03-15 08:14:02.000  info=[00, 00, 00, 00]
//!          class=Asynchronous errors  name=Cycle time exceeded
//! ------------------------------------------------------------
//! Disconnected
//! ```

use rust7::{describe_event, S7Client, S7_SZL_CPU_ID};

fn print_separator() {
    println!("{}", "-".repeat(60));
}

fn main() {
    let ip = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1".to_string());

    println!("rust7 — Diagnostic Buffer Example");
    print_separator();
    println!("Connecting to PLC at {} ...", ip);

    let mut client = S7Client::new();
    match client.connect_s71200_1500(&ip) {
        Ok(_) => {
            println!("Connected");
            println!("PDU negotiated : {} bytes", client.pdu_length);
            println!("Job time (ms)  : {:.3}", client.last_time);
        }
        Err(e) => {
            eprintln!("Connection failed: {}", e);
            return;
        }
    }

    // ── 1. CPU / module identification (SZL 0x001C) ──────────────────────────
    print_separator();
    println!("Reading CPU info (SZL 0x001C) ...");
    match client.read_cpu_info() {
        Ok(info) => {
            println!("Success!");
            println!("Job time (ms)  : {:.3}", client.last_time);
            println!("Chunks         : {}", client.chunks);
            println!("Module type    : {}", info.module_type_name);
            println!("Module name    : {}", info.module_name);
            println!("AS name        : {}", info.as_name);
            println!("Serial number  : {}", info.serial_number);
            println!("Copyright      : {}", info.copyright);
        }
        Err(e) => eprintln!("read_cpu_info failed: {}", e),
    }

    // ── 2. Raw SZL read — CPU identification (SZL 0x0011) ───────────────────
    print_separator();
    println!("Reading raw SZL 0x{:04X} (CPU identification) ...", S7_SZL_CPU_ID);
    match client.read_szl(S7_SZL_CPU_ID, 0) {
        Ok(szl) => {
            println!("Success!");
            println!("Job time (ms)  : {:.3}", client.last_time);
            println!("Chunks         : {}", client.chunks);
            println!(
                "SZL header     : length_dr={} n_dr={}",
                szl.header.length_dr, szl.header.n_dr
            );
            println!("Payload ({} bytes):", szl.data.len());
            for (i, chunk) in szl.data.chunks(16).enumerate() {
                print!("  {:04X}: ", i * 16);
                for byte in chunk {
                    print!("{:02X} ", byte);
                }
                println!();
            }
        }
        Err(e) => eprintln!("read_szl(0x{:04X}) failed: {}", S7_SZL_CPU_ID, e),
    }

    // ── 3. Structured diagnostic buffer (SZL 0x00A0) ─────────────────────────
    print_separator();
    println!("Reading diagnostic buffer (SZL 0x00A0) ...");
    match client.read_diagnostic_buffer() {
        Ok(entries) => {
            println!("Success!");
            println!("Job time (ms)  : {:.3}", client.last_time);
            println!("Chunks         : {}", client.chunks);
            println!("Entries        : {}", entries.len());

            if entries.is_empty() {
                println!("(diagnostic buffer is empty)");
            } else {
                println!();
                for (i, entry) in entries.iter().enumerate() {
                    let ts = match &entry.timestamp {
                        Some(dt) => format!(
                            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                            dt.year, dt.month, dt.day,
                            dt.hour, dt.minute, dt.second, dt.millisecond
                        ),
                        None => "(invalid timestamp)".to_string(),
                    };
                    let ev = describe_event(entry.event_id);
                    println!(
                        "  [{:>3}] event=0x{:04X}  ts={}  info={:02X?}",
                        i, entry.event_id, ts, entry.info
                    );
                    println!(
                        "         class={}{}",
                        ev.class,
                        ev.name.map(|n| format!("  name={n}")).unwrap_or_default()
                    );
                }
            }
        }
        Err(e) => eprintln!("read_diagnostic_buffer failed: {}", e),
    }

    print_separator();
    client.disconnect();
    println!("Disconnected");
}
