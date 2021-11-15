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

use aarch64_esr_decoder::{decode, parse_number, FieldInfo};
use std::env;
use std::process::exit;

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(error_code) => exit(error_code),
    };

    let esr = parse_number(&args.esr).unwrap();
    let decoded = decode(esr).unwrap();
    println!("ESR {:#034x}:", esr);
    print_decoded(&decoded, args.verbose, 0);
}

fn print_decoded(fields: &[FieldInfo], verbose: bool, level: usize) {
    let indentation = " ".repeat(level * 2);
    for field in fields {
        let verbose_name = match field.long_name {
            Some(long_name) if verbose => format!(" ({})", long_name),
            _ => "".to_string(),
        };
        if field.width == 1 {
            println!(
                "{}{:02}     {}{}",
                indentation, field.start, field, verbose_name
            );
        } else {
            println!(
                "{}{:02}..{:02} {}",
                indentation,
                field.start,
                field.start + field.width - 1,
                field
            );
        }
        if let Some(description) = &field.description {
            println!("{}  # {}", indentation, description);
        }

        print_decoded(&field.subfields, verbose, level + 1);
    }
}

/// Parse and return command-line arguments, or an error code to return.
fn parse_args() -> Result<Args, i32> {
    let mut args: Vec<_> = env::args().collect();
    match args.len() {
        2 => Ok(Args {
            verbose: false,
            esr: args.remove(1),
        }),
        3 => Ok(Args {
            verbose: true,
            esr: args.remove(2),
        }),
        _ => {
            eprintln!("Usage:");
            eprintln!("  {} [-v] <ESR value>", args[0]);
            Err(1)
        }
    }
}

/// Command-line arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Args {
    verbose: bool,
    esr: String,
}
