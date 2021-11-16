# Changelog

## Unreleased

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

## 0.1.0

Initial release.
