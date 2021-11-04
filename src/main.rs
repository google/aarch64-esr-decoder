// Copyright 2021 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use aarch64_esr_decoder::{decode, Decoded};
use std::{env, num::ParseIntError};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage:");
        eprintln!("  {} <ESR value>", args[0]);
        return;
    }

    let esr = parse_number(&args[1]).unwrap();
    let decoded = decode(esr).unwrap();
    println!("ESR {:#034x}:", esr);
    print_decoded(&decoded, 0);
}

fn print_decoded(decoded: &Decoded, level: usize) {
    let indentation = " ".repeat(level * 2);
    if let Some(description) = &decoded.description {
        println!("{}# {}", indentation, description);
    }
    for field in &decoded.fields {
        if field.width == 1 {
            println!("{}{:02}     {}", indentation, field.start, field);
        } else {
            println!(
                "{}{:02}..{:02} {}",
                indentation,
                field.start,
                field.start + field.width - 1,
                field
            );
        }
        if let Some(field_decoded) = &field.decoded {
            print_decoded(field_decoded, level + 1);
        }
    }
}

/// Parse a decimal or hexadecimal number.
fn parse_number(s: &str) -> Result<u64, ParseIntError> {
    if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16)
    } else {
        s.parse()
    }
}
