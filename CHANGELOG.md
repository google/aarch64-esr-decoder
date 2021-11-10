# Changelog

## Unreleased

### Breaking changes

- Removed `Decoded` struct, inlined its fields into `FieldInfo`.

### Bugfixes

- Added CRn field which was missing from MCR or MRC accesses.

### New features

- Added ISS decoding for SVE exceptions.

## 0.1.0

Initial release.
