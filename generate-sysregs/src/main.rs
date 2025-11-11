// Copyright 2025 Google LLC
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

use arm_sysregs_json::RegisterEntry;
use clap::Parser;
use eyre::Report;
use std::{fs::read_to_string, path::PathBuf};

fn main() -> Result<(), Report> {
    let args = Args::parse();
    let registers: Vec<RegisterEntry> =
        serde_json::from_str(&read_to_string(&args.registers_json)?)?;
    println!(
        "Read {} system registers from {}",
        registers.len(),
        args.registers_json.display()
    );

    Ok(())
}

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Path to JSON system registers file.
    registers_json: PathBuf,
}
