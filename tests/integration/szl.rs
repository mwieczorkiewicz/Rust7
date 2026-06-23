// Integration tests for SZL (System Status List) reads.
//
// fbarresi/softplc may not implement ROSCTR 0x07 (Userdata) / SZL at all.
// All tests are marked #[ignore] until verified against the container.
// To probe: `cargo test --test integration szl -- --ignored` and inspect the output.
// If the PLC answers, remove #[ignore] from the passing tests and document the result
// in CLAUDE.md under "Integration tests".

use super::common;
use rust7::{S7_SZL_CPU_ID, S7_SZL_DIAG_BUFFER};

#[test]
#[ignore = "probe: fbarresi/softplc SZL support is unverified — run manually to check"]
fn read_szl_cpu_id_returns_non_empty_header() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    match client.read_szl(S7_SZL_CPU_ID, 0) {
        Ok(szl) => {
            assert!(
                szl.header.length_dr > 0 || szl.header.n_dr > 0 || !szl.data.is_empty(),
                "SZL 0x0011 returned an empty response: {:?}",
                szl.header
            );
            println!(
                "SZL 0x0011: length_dr={}, n_dr={}, data_len={}",
                szl.header.length_dr,
                szl.header.n_dr,
                szl.data.len()
            );
        }
        Err(e) => {
            // Expected when SoftPLC does not support ROSCTR 0x07; mark test #[ignore].
            panic!("read_szl(0x0011) failed — SoftPLC may not support SZL: {e}");
        }
    }
}

#[test]
#[ignore = "probe: fbarresi/softplc SZL support is unverified — run manually to check"]
fn read_szl_diag_buffer_returns_result() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    match client.read_szl(S7_SZL_DIAG_BUFFER, 0) {
        Ok(szl) => {
            println!(
                "SZL 0x00A0: length_dr={}, n_dr={}, data_len={}",
                szl.header.length_dr,
                szl.header.n_dr,
                szl.data.len()
            );
            // Data length should be a multiple of 20 (each diagnostic entry is 20 bytes)
            if szl.header.length_dr > 0 {
                assert_eq!(
                    szl.data.len() % szl.header.length_dr as usize,
                    0,
                    "record data not aligned to length_dr"
                );
            }
        }
        Err(e) => {
            panic!("read_szl(0x00A0) failed — SoftPLC may not support SZL: {e}");
        }
    }
}

#[test]
#[ignore = "probe: fbarresi/softplc SZL support is unverified — run manually to check"]
fn read_diagnostic_buffer_returns_vec() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    match client.read_diagnostic_buffer() {
        Ok(entries) => {
            println!("Diagnostic buffer: {} entries", entries.len());
            for (i, e) in entries.iter().take(3).enumerate() {
                println!("  [{i}] event_id=0x{:04X} ts={:?}", e.event_id, e.timestamp);
            }
            // Even an empty diagnostic buffer is a valid result
        }
        Err(e) => {
            panic!("read_diagnostic_buffer failed: {e}");
        }
    }
}

#[test]
#[ignore = "probe: fbarresi/softplc SZL support is unverified — run manually to check"]
fn read_cpu_info_returns_some_strings() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    match client.read_cpu_info() {
        Ok(info) => {
            println!(
                "CpuInfo: type={:?} name={:?} as={:?} serial={:?}",
                info.module_type_name, info.module_name, info.as_name, info.serial_number
            );
            // At least one field should be non-empty if SZL 0x001C is supported
            assert!(
                !info.module_type_name.is_empty()
                    || !info.module_name.is_empty()
                    || !info.serial_number.is_empty(),
                "all CpuInfo fields empty — SZL 0x001C returned no known records"
            );
        }
        Err(e) => {
            panic!("read_cpu_info failed: {e}");
        }
    }
}

#[test]
fn read_work_memory_returns_records() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    match client.read_work_memory() {
        Ok(records) => {
            println!("Work memory: {} records", records.len());
            for r in &records {
                println!(
                    "  index={:#06X} area_type={:#06X} total={} used={}",
                    r.index, r.area_type, r.total_bytes, r.used_bytes
                );
            }
        }
        Err(e) => panic!("read_work_memory failed: {e}"),
    }
}

#[test]
#[ignore = "fbarresi/softplc returns IsoInvalidTelegram for SZL 0x0194 — test against a real PLC"]
fn read_cycle_time_returns_info() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    match client.read_cycle_time() {
        Ok(ct) => {
            println!(
                "Cycle time: count={} min={:.1}ms max={:.1}ms current={:.1}ms",
                ct.ob1_count, ct.min_ms, ct.max_ms, ct.current_ms
            );
            // All times must be non-negative
            assert!(ct.min_ms >= 0.0);
            assert!(ct.max_ms >= 0.0);
            assert!(ct.current_ms >= 0.0);
            // min <= max (unless PLC is in STOP and all are zero)
            if ct.max_ms > 0.0 {
                assert!(
                    ct.min_ms <= ct.max_ms,
                    "min_ms ({}) > max_ms ({})",
                    ct.min_ms,
                    ct.max_ms
                );
            }
        }
        Err(e) => panic!("read_cycle_time failed: {e}"),
    }
}

#[test]
#[ignore = "probe: fbarresi/softplc SZL support is unverified — run manually to check"]
fn read_szl_sets_last_time_and_chunks() {
    let container = common::start_softplc();
    let s7_port = container
        .get_host_port_ipv4(102)
        .expect("port 102 not mapped");
    let mut client = common::connect_client(s7_port);

    client.read_szl(S7_SZL_CPU_ID, 0).expect("read_szl failed");
    assert!(
        client.last_time > 0.0,
        "last_time should be set after a successful SZL read"
    );
    assert!(client.chunks >= 1, "chunks should be at least 1");
}
