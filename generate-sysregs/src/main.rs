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

mod config;
mod json_input;
mod output;

use crate::{
    config::Config,
    output::{write_fake, write_lib},
};
use arm_sysregs_json::{Register, RegisterEntry};
use clap::Parser;
use eyre::Report;
use std::{
    fs::{File, read_to_string},
    ops::Range,
    path::PathBuf,
};

fn main() -> Result<(), Report> {
    pretty_env_logger::init();
    let args = Args::parse();
    let config: Config = toml::from_str(&read_to_string(&args.config_toml)?)?;
    let registers: Vec<RegisterEntry> =
        serde_json::from_str(&read_to_string(&args.registers_json)?)?;
    println!(
        "Read {} system registers from {}",
        registers.len(),
        args.registers_json.display()
    );
    let output_lib = File::create(args.output_directory.join("lib.rs"))?;
    let output_fake = File::create(args.output_directory.join("fake.rs"))?;
    let registers_filter = config.registers.keys().collect::<Vec<_>>();
    let mut register_infos = generate_all(&registers, &registers_filter);
    add_descriptions(&mut register_infos, &config);
    write_lib(&output_lib, &register_infos)?;
    write_fake(&output_fake, &register_infos)?;

    Ok(())
}

fn add_descriptions(registers: &mut Vec<RegisterInfo>, config: &Config) {
    for register in registers {
        if let Some(register_config) = config.registers.get(&register.name) {
            for field in &mut register.fields {
                if let Some(description) = register_config.field_descriptions.get(&field.name) {
                    field.description = Some(description.clone());
                }
            }
        }
    }
}

fn generate_all(registers: &[RegisterEntry], registers_filter: &[&String]) -> Vec<RegisterInfo> {
    let mut register_infos = Vec::new();

    for register in registers {
        match register {
            RegisterEntry::Register(register) => {
                if filter_matches(registers_filter, register) {
                    register_infos.push(RegisterInfo::from_json_register(register));
                }
            }
            _ => {}
        }
    }

    register_infos
}

fn filter_matches(filter: &[&String], register: &Register) -> bool {
    filter
        .iter()
        .any(|filter_entry| register.name == **filter_entry)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterField {
    /// The name of the field.
    pub name: String,
    /// The description of the field, if available.
    pub description: Option<String>,
    /// The index of the least significant bit of the field.
    pub index: u32,
    /// The width of the field in bits.
    pub width: u32,
    /// Information about the array, if it is an array field.
    pub array_info: Option<ArrayInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArrayInfo {
    /// The range of entries in the array.
    pub indices: Range<u32>,
    /// The placeholder variable name.
    pub index_variable: String,
}

impl ArrayInfo {
    fn placeholder(&self) -> String {
        format!("<{}>", self.index_variable)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterInfo {
    pub name: String,
    pub width: u32,
    pub fields: Vec<RegisterField>,
    /// All the bits which are RES1.
    pub res1: u64,
    pub read: Option<Safety>,
    pub write: Option<Safety>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Path to config toml file.
    config_toml: PathBuf,
    /// Path to JSON system registers file.
    registers_json: PathBuf,
    /// Path to output directory.
    output_directory: PathBuf,
}

/// Returns a value with the given number of 1 bits, starting at the least significant bit.
const fn ones(n: u32) -> u64 {
    u64::MAX >> (64 - n)
}
