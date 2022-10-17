# Changelog

## 0.2.2

### New features

- Added support for decoding SMCCC function IDs.
- Added support for MIDR and SMCCC to command-line tool.

## 0.2.1

### Bugfixes

- Print long name for multi-bit fields in command-line app (they were only being printed for single
  bit fields).

### New features

- Added support for decoding MIDR values too.
- Added HVC and SMC ECs for `ESR_EL2`.

## 0.2.0

### Breaking changes

- Removed `Decoded` struct, inlined its fields into `FieldInfo`.
- `DecodeError` variants changed.
- Added `long_name` to `FieldInfo` struct.

### Bugfixes

- Added CRn field which was missing from MCR or MRC accesses.

### New features

- Added ISS decoding for SVE exceptions.
- Added ISS decoding for LD64B/ST64B\* exceptions.
- Added ISS decoding for Branch Target Exception.
- Added ISS decoding for HVC and SVC exceptions.
- Added ISS decoding for MRS and MSR exceptions, including system register names.
- Added ISS decoding for Pointer Authentication failures.
- Added ISS decoding for floating-point exceptions.
- Added ISS decoding for SError interrupts.
- Added ISS decoding for Breakpoint, Watchpoint, Software Step and Vector Catch exceptions.

## 0.1.0

Initial release.
