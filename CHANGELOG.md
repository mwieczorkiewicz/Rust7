# Changelog

---

## [Unreleased]

### Added
- `read_work_memory()` — reads work memory area sizes from SZL `0x0013` (`WorkMemoryRecord`)
- `read_cycle_time()` — reads OB1 scan cycle time statistics from SZL `0x0194` (`CycleTimeInfo`)
- `S7_SZL_WORK_MEMORY` (`0x0013`) and `S7_SZL_CYCLE_TIME` (`0x0194`) constants
- Integration test for `read_work_memory` (passes against `fbarresi/softplc`)

---

## [0.1.2] - 2025-08-15

### Added
- Added parameter check
- Added `InvalidFunParam`

### Modified
- Excluded /target folder from deploy

## [0.1.1] - 2025-08-14
- Initial release
