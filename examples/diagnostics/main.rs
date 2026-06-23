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

use rust7::{S7Client, S7_SZL_CPU_ID};

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
                    println!(
                        "  [{:>3}] event=0x{:04X}  ts={}  info={:02X?}",
                        i, entry.event_id, ts, entry.info
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
