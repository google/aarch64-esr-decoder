# Changelog

## Unreleased

### Breaking changes

- Removed `Decoded` struct, inlined its fields into `FieldInfo`.
- `DecodeError` variants changed.

### Bugfixes

- Added CRn field which was missing from MCR or MRC accesses.

### New features

- Added ISS decoding for SVE exceptions.
- Added ISS decoding for LD64B/ST64B\* exceptions.

## 0.1.0

Initial release.
