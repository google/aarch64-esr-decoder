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

use crate::output::{write_fake, write_lib};
use arm_sysregs_json::{
    ConditionalField, ConstantField, Field, FieldEntry, Register, RegisterEntry,
};
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
    let output_lib = File::create(args.output_directory.join("lib.rs"))?;
    let output_fake = File::create(args.output_directory.join("fake.rs"))?;
    let registers_filter = args
        .registers
        .as_ref()
        .map(|registers| registers.split(',').collect::<Vec<_>>());
    let register_infos = generate_all(&registers, registers_filter.as_deref());
    write_lib(&output_lib, &register_infos)?;
    write_fake(&output_fake, &register_infos)?;

    Ok(())
}

fn generate_all(
    registers: &[RegisterEntry],
    registers_filter: Option<&[&str]>,
) -> Vec<RegisterInfo> {
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

fn filter_matches(filter: Option<&[&str]>, register: &Register) -> bool {
    if let Some(filter) = filter {
        filter
            .iter()
            .any(|filter_entry| register.name == *filter_entry)
    } else {
        true
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RegisterField {
    /// The name of the field.
    pub name: String,
    /// The index of the least significant bit of the field.
    pub index: u32,
    /// The width of the field in bits.
    pub width: u32,
}

impl RegisterField {
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
            FieldEntry::ImplementationDefined(implementation_defined_field) => {
                info!("Skipping implementation defined field {implementation_defined_field:?}");
                None
            }
            FieldEntry::ConditionalField(field) => {
                trace!(
                    "  Conditional field: {:?}, {:?}",
                    field.name, field.rangeset
                );
                Self::from_conditional_field(field)
            }
            FieldEntry::Array(_array_field) => todo!(),
            FieldEntry::ConstantField(constant_field) => {
                info!(
                    "  Constant field: {:?} {:?}",
                    constant_field.name, constant_field.rangeset
                );
                Self::from_constant_field(constant_field)
            }
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
                    // If different options give a different RegisterField, ignore them all to be
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
            let name = field.name.clone().unwrap();
            Some(RegisterField {
                name,
                index: offset + range.start,
                width: range.width,
            })
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }

    fn from_constant_field(field: &ConstantField) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let name = field.name.clone().unwrap();
            Some(RegisterField {
                name,
                index: range.start,
                width: range.width,
            })
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
    pub fields: Vec<RegisterField>,
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
        let mut fields = Vec::new();
        let mut writable = false;
        for fieldset in &register.fieldsets {
            for field_entry in &fieldset.values {
                fields.extend(RegisterField::from_field_entry(field_entry));
                if matches!(
                    field_entry,
                    FieldEntry::Field(_) | FieldEntry::ConditionalField(_)
                ) {
                    writable = true;
                }
            }
        }
        fields.sort_by_key(|field| field.index);
        fields.dedup();
        RegisterInfo {
            name: register.name.clone(),
            // TODO
            width: 64,
            fields,
            read: Some(Safety::Safe),
            // TODO
            write: if writable { Some(Safety::Unsafe) } else { None },
        }
    }
}

#[derive(Clone, Debug, Parser)]
struct Args {
    /// Path to JSON system registers file.
    registers_json: PathBuf,
    /// Path to output directory.
    output_directory: PathBuf,
    /// List of registers to include
    #[arg(long)]
    registers: Option<String>,
}
