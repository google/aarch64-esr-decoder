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

mod json_input;
mod output;

use crate::output::{write_fake, write_lib};
use arm_sysregs_json::{Register, RegisterEntry};
use clap::Parser;
use eyre::Report;
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
    add_descriptions(&mut register_infos, &FIELD_DESCRIPTIONS);
    write_lib(&output_lib, &register_infos)?;
    write_fake(&output_fake, &register_infos)?;

    Ok(())
}

fn add_descriptions(
    registers: &mut Vec<RegisterInfo>,
    descriptions: &BTreeMap<(&str, &str), &str>,
) {
    for register in registers {
        for field in &mut register.fields {
            if let Some(description) =
                descriptions.get(&(register.name.as_str(), field.name.as_str()))
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
