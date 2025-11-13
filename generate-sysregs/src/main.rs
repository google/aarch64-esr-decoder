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

mod output;

use crate::output::write_all;
use arm_sysregs_json::{ConditionalField, Field, FieldEntry, Register, RegisterEntry};
use clap::Parser;
use eyre::Report;
use log::{info, trace};
use std::{
    fs::{File, read_to_string},
    path::PathBuf,
};

fn main() -> Result<(), Report> {
    pretty_env_logger::init();
    let args = Args::parse();
    let registers: Vec<RegisterEntry> =
        serde_json::from_str(&read_to_string(&args.registers_json)?)?;
    println!(
        "Read {} system registers from {}",
        registers.len(),
        args.registers_json.display()
    );
    let mut output_file = File::create(args.output_file)?;
    generate_all(&registers, &mut output_file)?;

    Ok(())
}

fn generate_all(registers: &[RegisterEntry], output_file: &File) -> Result<(), Report> {
    let mut register_infos = Vec::new();

    for register in registers {
        match register {
            RegisterEntry::Register(register) => {
                if register.name == "SCR_EL3" {
                    let register_info = RegisterInfo::from_json_register(register);
                    println!("{register_info:#?}");
                    register_infos.push(register_info);
                }
            }
            _ => {}
        }
    }

    write_all(output_file, &register_infos)?;

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterBit {
    pub name: String,
    pub index: u32,
}

impl RegisterBit {
    fn from_field_entry(field_entry: &FieldEntry) -> Option<Self> {
        match field_entry {
            FieldEntry::Field(field) => {
                trace!("  Field: {:?} {:?}", field.name, field.rangeset);
                Self::from_field(field, 0)
            }
            FieldEntry::Reserved(field) => {
                trace!("  Reserved field: {:?}", field.rangeset);
                None
            }
            FieldEntry::ImplementationDefined(_implementation_defined_field) => todo!(),
            FieldEntry::ConditionalField(field) => {
                trace!(
                    "  Conditional field: {:?}, {:?}",
                    field.name, field.rangeset
                );
                Self::from_conditional_field(field)
            }
            FieldEntry::Array(_array_field) => todo!(),
            FieldEntry::ConstantField(_constant_field) => todo!(),
            FieldEntry::Dynamic(_dynamic_field) => todo!(),
            FieldEntry::Vector(_vector_field) => todo!(),
        }
    }

    fn from_conditional_field(field: &ConditionalField) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let mut bit = None;
            for field in &field.fields {
                trace!(
                    "    Field: {:?} {:?}",
                    field.field.name, field.field.rangeset
                );
                if bit.is_none() {
                    bit = Self::from_field(&field.field, range.start);
                } else if Self::from_field(&field.field, range.start) != bit {
                    // If different options give a different RegisterBit, ignore them all to be
                    // safe.
                    return None;
                }
            }
            bit
        } else {
            info!(
                "Skipping conditional field with multiple ranges {:?}",
                field.rangeset
            );
            None
        }
    }

    fn from_field(field: &Field, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            if range.width == 1 {
                let name = field.name.clone().unwrap();
                Some(RegisterBit {
                    name,
                    index: offset + range.start,
                })
            } else {
                info!("Skipping multi-bit field {:?} {:?}", field.name, range);
                None
            }
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterInfo {
    pub name: String,
    pub width: u32,
    pub bits: Vec<RegisterBit>,
    pub read: Option<Safety>,
    pub write: Option<Safety>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Safety {
    Safe,
    Unsafe,
}

impl RegisterInfo {
    fn from_json_register(register: &Register) -> RegisterInfo {
        trace!("{:#?}", register);
        let mut bits = Vec::new();
        for fieldset in &register.fieldsets {
            println!("fieldset: {:?}", fieldset.name);
            for field_entry in &fieldset.values {
                bits.extend(RegisterBit::from_field_entry(field_entry));
            }
        }
        RegisterInfo {
            name: register.name.clone(),
            // TODO
            width: 64,
            bits,
            // TODO
            read: Some(Safety::Safe),
            // TODO
            write: None,
        }
    }
}

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Path to JSON system registers file.
    registers_json: PathBuf,
    /// Path to output file.
    output_file: PathBuf,
}
