# aarch64-esr-decoder

[![crates.io page](https://img.shields.io/crates/v/aarch64-esr-decoder.svg)](https://crates.io/crates/aarch64-esr-decoder)
[![docs.rs page](https://docs.rs/aarch64-esr-decoder/badge.svg)](https://docs.rs/aarch64-esr-decoder)

A small utility for decoding aarch64 ESR register values.

This is not an officially supported Google product.

## Usage

Install a Debian package from the
[latest release](https://github.com/google/aarch64-esr-decoder/releases/tag/0.2.0), install with
`cargo install aarch64-esr-decoder`, or
[try the web version](https://google.github.io/aarch64-esr-decoder/).

For the command-line version, just pass the ESR value you're interested in as a parameter, in
decimal or hexadecimal:

```
$ aarch64-esr-decoder 0x96000050
ESR 0x00000000000000000000000096000050:
# Data Abort taken without a change in Exception level
37..63 RES0: 0x0000000 0b000000000000000000000000000
32..36 ISS2: 0x00 0b00000
26..31 EC: 0x25 0b100101
  # Data Abort taken without a change in Exception level
25     IL: true
  # 32-bit instruction trapped
00..24 ISS: 0x0000050 0b0000000000000000001010000
  24     ISV: false
    # No valid instruction syndrome
  14..23 RES0: 0x000 0b0000000000
  13     VNCR: false
  11..12 SET: 0x0 0b00
    # Recoverable state (UER)
  10     FnV: false
    # FAR is valid
  09     EA: false
  08     CM: false
  07     S1PTW: false
  06     WnR: true
    # Abort caused by writing to memory
  00..05 DFSC: 0x10 0b010000
    # Synchronous External abort, not on translation table walk or hardware update of translation table.
```

For long field names, add `-v`.

## License

Licensed under the [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0).

## Contributing

If you want to contribute to the project, see details of
[how we accept contributions](CONTRIBUTING.md).
