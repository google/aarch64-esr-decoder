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
    ArrayField, ConditionalField, ConstantField, Field, FieldEntry, Register, RegisterEntry,
};
use clap::Parser;
use eyre::Report;
use log::{info, trace};
use std::{
    collections::BTreeMap,
    fs::{File, read_to_string},
    ops::Range,
    path::PathBuf,
    sync::LazyLock,
};

static FIELD_DESCRIPTIONS: LazyLock<BTreeMap<(&'static str, &'static str), &'static str>> =
    LazyLock::new(|| {
        [
            (("SCR_EL3", "NS"), "Non-secure."),
            (("SCR_EL3", "IRQ"), "Take physical IRQs at EL3."),
            (("SCR_EL3", "FIQ"), "Take physical FIQs at EL3."),
            (
                ("SCR_EL3", "EA"),
                "Take external abort and SError exceptions at EL3.",
            ),
            (("SCR_EL3", "SMD"), "Disable SMC instructions."),
            (("SCR_EL3", "HCE"), "Enable HVC instructions."),
            (
                ("SCR_EL3", "SIF"),
                "Disable execution from non-secure memory.",
            ),
            (("SCR_EL3", "RW"), "Enable AArch64 in lower ELs."),
            (("SCR_EL3", "ST"), "Trap physical secure timer to EL3."),
            (("SCR_EL3", "TWI"), "Trap WFI to EL3."),
            (("SCR_EL3", "TWE"), "Trap WFE to EL3."),
            (("SCR_EL3", "TLOR"), "Trap LOR register access to EL3."),
            (
                ("SCR_EL3", "TERR"),
                "Trap error record register access to EL3.",
            ),
            (("SCR_EL3", "APK"), "Don't trap PAC key registers to EL3."),
            (("SCR_EL3", "API"), "Don't trap PAuth instructions to EL3."),
            (("SCR_EL3", "EEL2"), "Enable Secure EL2."),
            (
                ("SCR_EL3", "EASE"),
                "Synchronous external aborts are taken as SErrors.",
            ),
            (("SCR_EL3", "NMEA"), "Take SError exceptions at EL3."),
            (("SCR_EL3", "FIEN"), "Enable fault injection at lower ELs."),
            (("SCR_EL3", "TID3"), "Trap ID group 3 registers to EL3."),
            (("SCR_EL3", "TID5"), "Trap ID group 5 register to EL3."),
            (("SCR_EL3", "EnSCXT"), "Enable SCXT at lower ELs."),
            (("SCR_EL3", "ATA"), "Enable memory tagging at lower ELs."),
            (("SCR_EL3", "FGTEn"), "Enable fine-grained traps to EL2."),
            (("SCR_EL3", "ECVEn"), "Enable access to CNTPOFF_EL2."),
            (
                ("SCR_EL3", "TWEDEn"),
                "Enable a configurable delay for WFE traps.",
            ),
            (("SCR_EL3", "TME"), "Enable access to TME at lower ELs."),
            (
                ("SCR_EL3", "AMVOFFEN"),
                "Enable acivity monitors virtual offsets.",
            ),
            (("SCR_EL3", "EnAS0"), "Enable ST64BV0 at lower ELs."),
            (("SCR_EL3", "ADEn"), "Enable ACCDATA_EL1 at lower ELs."),
            (("SCR_EL3", "HXEn"), "Enable HCRX_EL2."),
            (("SCR_EL3", "GCSEn"), "Enable gaurded control stack."),
            (("SCR_EL3", "TRNDR"), "Trap RNDR and RNDRRS to EL3."),
            (("SCR_EL3", "EnTP2"), "Enable TPIDR2_EL0 at lower ELs."),
            (
                ("SCR_EL3", "RCWMASKEn"),
                "Enable RCW and RCWS mask registers at lower ELs.",
            ),
            (
                ("SCR_EL3", "TCR2En"),
                "Enable TCR2_ELx registers at lower ELs.",
            ),
            (
                ("SCR_EL3", "SCTLR2En"),
                "Enable SCTLR2_ELx rogisters at lower ELs.",
            ),
            (
                ("SCR_EL3", "PIEn"),
                "Enable permission indirection and overlay registers at lower ELs.",
            ),
            (
                ("SCR_EL3", "AIEn"),
                "Enable MAIR2_ELx and AMAIR2_ELx at lower ELs.",
            ),
            (
                ("SCR_EL3", "D128En"),
                "Enable 128-bit system registers at  lower ELs.",
            ),
            (("SCR_EL3", "GPF"), "Route GPFs to EL3."),
            (("SCR_EL3", "MECEn"), "Enable MECID registers at EL2."),
            (("SCR_EL3", "EnFPM"), "Enable access to FPMR at lower ELs."),
            (
                ("SCR_EL3", "TMEA"),
                "Take synchronous external abort and physical SError exception to EL3.",
            ),
            (
                ("SCR_EL3", "TWERR"),
                "Trap writes to Error Record registers to EL3.",
            ),
            (
                ("SCR_EL3", "PFAREn"),
                "Enable access to physical fault address registers at lower ELs.",
            ),
            (
                ("SCR_EL3", "SRMASKEn"),
                "Enable access to mask registers at lower ELs.",
            ),
            (
                ("SCR_EL3", "EnIDCP128"),
                "Enable implementation-defined 128-bit system registers.",
            ),
            (
                ("SCR_EL3", "DSE"),
                "A delegated SError exception is pending.",
            ),
            (("SCR_EL3", "EnDSE"), "Enable delegated SError exceptions."),
            (("SCR_EL3", "FGTEn2"), "Enable fine-grained traps to EL2."),
            (
                ("SCR_EL3", "HDBSSEn"),
                "Enable HDBSSBR_EL2 and HDBSSPROD_EL2 registers at EL2.",
            ),
            (
                ("SCR_EL3", "HACDBSEn"),
                "Enable HACDBSBR_EL2 and HACDBSCONS_EL2 registers at EL2.",
            ),
            (("SCR_EL3", "NSE"), "Non-secure realm world bit."),
            (
                ("CLIDR_EL1", "LoC"),
                "Level of Coherence for the cache hierarchy.",
            ),
        ]
        .into_iter()
        .collect()
    });

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
    let mut register_infos = generate_all(&registers, registers_filter.as_deref());
    add_descriptions(&mut register_infos);
    write_lib(&output_lib, &register_infos)?;
    write_fake(&output_fake, &register_infos)?;

    Ok(())
}

fn add_descriptions(registers: &mut Vec<RegisterInfo>) {
    for register in registers {
        for field in &mut register.fields {
            if let Some(description) =
                FIELD_DESCRIPTIONS.get(&(register.name.as_str(), field.name.as_str()))
            {
                field.description = Some(description.to_string());
            }
        }
    }
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

impl RegisterField {
    fn from_field_entry(field_entry: &FieldEntry, offset: u32) -> Option<Self> {
        match field_entry {
            FieldEntry::Field(field) => {
                trace!("  Field: {:?} {:?}", field.name, field.rangeset);
                Self::from_field(field, offset)
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
                Self::from_conditional_field(field, offset)
            }
            FieldEntry::Array(field) => {
                info!(
                    "  Array field: {:?}, {:?}, {}, {:?}",
                    field.name, field.rangeset, field.index_variable, field.indexes
                );
                Self::from_array_field(field, offset)
            }
            FieldEntry::ConstantField(constant_field) => {
                info!(
                    "  Constant field: {:?} {:?}",
                    constant_field.name, constant_field.rangeset
                );
                Self::from_constant_field(constant_field, offset)
            }
            FieldEntry::Dynamic(_dynamic_field) => todo!(),
            FieldEntry::Vector(_vector_field) => todo!(),
        }
    }

    fn from_conditional_field(field: &ConditionalField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let mut bit = None;
            for field in &field.fields {
                if bit.is_none() {
                    bit = Self::from_field_entry(&field.field, offset + range.start);
                } else if Self::from_field_entry(&field.field, offset + range.start) != bit {
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
                description: None,
                index: offset + range.start,
                width: range.width,
                array_info: None,
            })
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }

    fn from_array_field(field: &ArrayField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            if let [array_range] = field.indexes.as_slice() {
                let name = field.name.clone().unwrap();
                Some(RegisterField {
                    name,
                    description: None,
                    index: offset + range.start,
                    width: range.width / array_range.width,
                    array_info: Some(ArrayInfo {
                        indices: array_range.start..array_range.start + array_range.width,
                        index_variable: field.index_variable.clone(),
                    }),
                })
            } else {
                info!(
                    "Skipping field with multiple array indices {:?}",
                    field.rangeset
                );
                None
            }
        } else {
            info!("Skipping field with multiple ranges {:?}", field.rangeset);
            None
        }
    }

    fn from_constant_field(field: &ConstantField, offset: u32) -> Option<Self> {
        if let [range] = field.rangeset.as_slice() {
            let name = field.name.clone().unwrap();
            Some(RegisterField {
                name,
                description: None,
                index: offset + range.start,
                width: range.width,
                array_info: None,
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

impl RegisterInfo {
    fn from_json_register(register: &Register) -> RegisterInfo {
        trace!("{:#?}", register);
        let mut fields = Vec::new();
        let mut writable = false;
        let mut res1 = 0;
        for fieldset in &register.fieldsets {
            for field_entry in &fieldset.values {
                fields.extend(RegisterField::from_field_entry(field_entry, 0));
                if matches!(
                    field_entry,
                    FieldEntry::Field(_) | FieldEntry::ConditionalField(_)
                ) {
                    writable = true;
                }
                if let FieldEntry::Reserved(field) = field_entry
                    && field.value == "RES1"
                {
                    for range in &field.rangeset {
                        res1 |= ones(range.width) << range.start
                    }
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
            res1,
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

/// Returns a value with the given number of 1 bits, starting at the least significant bit.
const fn ones(n: u32) -> u64 {
    u64::MAX >> (64 - n)
}
