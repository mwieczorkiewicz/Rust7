#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub mod client;

pub use client::{
    S7Client, S7Error,
    CT_PG, CT_OP, CT_S7,
    S7_AREA_PE, S7_AREA_PA, S7_AREA_MK, S7_AREA_DB,
    S7_WL_BIT, S7_WL_BYTE,
    // SZL types
    Szl, SzlHeader, S7DateTime, DiagnosticEntry, CpuInfo,
    // SZL-ID constants
    S7_SZL_CPU_ID, S7_SZL_CPU_INFO, S7_SZL_DIAG_BUFFER, S7_SZL_CPU_STATUS,
};
