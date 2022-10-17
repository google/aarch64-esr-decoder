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

use aarch64_esr_decoder::{decode, decode_midr, decode_smccc, parse_number, FieldInfo};
use std::env;
use std::ops::Deref;
use std::process::exit;

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(error_code) => exit(error_code),
    };

    let value = parse_number(&args.value).unwrap();
    let decoded = match args.mode {
        Mode::Esr => {
            println!("ESR {:#034x}:", value);
            decode(value).unwrap()
        }
        Mode::Midr => {
            println!("MIDR {:#034x}:", value);
            decode_midr(value).unwrap()
        }
        Mode::Smccc => {
            println!("SMC ID {:#018x}:", value);
            decode_smccc(value).unwrap()
        }
    };
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
                "{}{:02}..{:02} {}{}",
                indentation,
                field.start,
                field.start + field.width - 1,
                field,
                verbose_name,
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
    let args: Vec<String> = env::args().collect();
    let args: Vec<&str> = args.iter().map(Deref::deref).collect();
    match args.as_slice() {
        [_, esr] => Ok(Args {
            verbose: false,
            mode: Mode::Esr,
            value: esr.to_string(),
        }),
        [_, "-v", esr] => Ok(Args {
            verbose: true,
            mode: Mode::Esr,
            value: esr.to_string(),
        }),
        [_, "midr", midr] => Ok(Args {
            verbose: false,
            mode: Mode::Midr,
            value: midr.to_string(),
        }),
        [_, "-v", "midr", midr] => Ok(Args {
            verbose: true,
            mode: Mode::Midr,
            value: midr.to_string(),
        }),
        [_, "smccc", smccc] => Ok(Args {
            verbose: false,
            mode: Mode::Smccc,
            value: smccc.to_string(),
        }),
        [_, "-v", "smccc", smccc] => Ok(Args {
            verbose: true,
            mode: Mode::Smccc,
            value: smccc.to_string(),
        }),
        _ => {
            eprintln!("Usage:");
            eprintln!("  {} [-v] <ESR value>", args[0]);
            eprintln!("  {} [-v] midr <MIDR value>", args[0]);
            eprintln!("  {} [-v] smccc <SMCCC function ID>", args[0]);
            Err(1)
        }
    }
}

/// Command-line arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
struct Args {
    verbose: bool,
    mode: Mode,
    value: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Mode {
    Esr,
    Midr,
    Smccc,
}
