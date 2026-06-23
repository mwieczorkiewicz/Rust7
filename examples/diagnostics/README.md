# Diagnostics Example

Demonstrates reading SZL (System Status List / *Systemzustandsliste*) data from a Siemens S7 PLC using rust7:

1. **CPU info** (`read_cpu_info`) — module type, name, serial number, copyright via SZL `0x001C`.
2. **Raw SZL** (`read_szl`) — raw hex dump of SZL `0x0011` (CPU identification).
3. **Diagnostic buffer** (`read_diagnostic_buffer`) — structured entries from SZL `0x00A0`, each with a decoded BCD timestamp and event ID.

---

## Quick Start

```bash
# Against a real PLC:
cargo run -- 192.168.0.100

# Against a locally-running SoftPLC container (see examples/docker/):
cargo run
```

`PLC_IP` defaults to `127.0.0.1` when omitted.

---

## SoftPLC compatibility

`fbarresi/softplc` does not implement ROSCTR `0x07` (Userdata), which is the PDU type used for all SZL reads. Against the SoftPLC container the three operations in this example will each print an error. Use a real S7-300 / S7-1200 / S7-1500 or a PLCSIM instance to exercise them.

---

## Prerequisites

- A Siemens S7-300, S7-1200, or S7-1500 PLC (or PLCSIM) reachable on port 102.
- For S7-1500: disable **Optimized block access** on any DB you want to read by byte, but SZL reads are not affected by that setting.

---

## Sample output

```
rust7 — Diagnostic Buffer Example
------------------------------------------------------------
Connecting to PLC at 192.168.0.100 ...
Connected
PDU negotiated : 480 bytes
Job time (ms)  : 12.345
------------------------------------------------------------
Reading CPU info (SZL 0x001C) ...
Success!
Job time (ms)  : 8.201
Chunks         : 1
Module type    : CPU 1214C DC/DC/DC
Module name    : My S7-1200
AS name        : Line1
Serial number  : S C-J5EA1234567
Copyright      : Original Siemens Equipment
------------------------------------------------------------
Reading raw SZL 0x0011 (CPU identification) ...
Success!
Job time (ms)  : 6.890
Chunks         : 1
SZL header     : length_dr=34 n_dr=1
Payload (34 bytes):
  0000: 00 01 43 50 55 20 31 32 31 34 43 20 44 43 2F 44
  0010: 43 2F 44 43 00 00 00 00 00 00 00 00 00 00 00 00
  0020: 00 00
------------------------------------------------------------
Reading diagnostic buffer (SZL 0x00A0) ...
Success!
Job time (ms)  : 9.450
Chunks         : 1
Entries        : 3

  [  0] event=0x4302  ts=2024-03-15 14:30:05.000  info=[00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
  [  1] event=0x4304  ts=2024-03-15 14:29:51.225  info=[00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
  [  2] event=0x4500  ts=2024-03-15 14:29:50.000  info=[00, 00, 00, 00, 00, 00, 00, 00, 00, 00]
------------------------------------------------------------
Disconnected
```
