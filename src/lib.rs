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

mod abort;
mod breakpoint;
mod bti;
mod common;
mod fp;
mod hvc;
mod ld64b;
mod ldc;
mod mcr;
mod msr;
mod pauth;
mod serror;
mod sve;
#[cfg(test)]
mod tests;
mod wf;

use abort::{decode_iss_data_abort, decode_iss_instruction_abort};
use bit_field::BitField;
use breakpoint::{
    decode_iss_breakpoint, decode_iss_breakpoint_vector_catch, decode_iss_software_step,
    decode_iss_watchpoint,
};
use bti::decode_iss_bti;
use fp::decode_iss_fp;
use hvc::decode_iss_hvc;
use ld64b::decode_iss_ld64b;
use ldc::decode_iss_ldc;
use mcr::{decode_iss_mcr, decode_iss_mcrr};
use msr::decode_iss_msr;
use pauth::decode_iss_pauth;
use serror::decode_iss_serror;
use std::fmt::{self, Debug, Display, Formatter};
use std::num::ParseIntError;
use sve::decode_iss_sve;
use thiserror::Error;
use wf::decode_iss_wf;

/// Information about a particular field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldInfo {
    /// The short name of the field, e.g. "ISS".
    pub name: &'static str,
    /// The long name of the field, e.g. "Instruction Specific Syndrome".
    pub long_name: Option<&'static str>,
    /// The index of the lowest bit of the field.
    pub start: usize,
    /// The number of bits in the field.
    pub width: usize,
    /// The value of the field.
    pub value: u64,
    /// A description explaining the field value, if available.
    pub description: Option<String>,
    /// Any sub-fields.
    pub subfields: Vec<FieldInfo>,
}

impl FieldInfo {
    fn get(
        register: u64,
        name: &'static str,
        long_name: Option<&'static str>,
        start: usize,
        end: usize,
    ) -> Self {
        let value = register.get_bits(start..end);
        Self {
            name,
            long_name,
            start,
            width: end - start,
            value,
            description: None,
            subfields: vec![],
        }
    }

    fn get_bit(
        register: u64,
        name: &'static str,
        long_name: Option<&'static str>,
        bit: usize,
    ) -> Self {
        Self::get(register, name, long_name, bit, bit + 1)
    }

    fn with_description(self, description: String) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    fn as_bit(&self) -> bool {
        assert!(self.width == 1);
        self.value == 1
    }

    /// Assuming this field has a width of exactly 1, describe it with the given function.
    ///
    /// Panics if `self.width != 1`.
    fn describe_bit<F>(self, describer: F) -> Self
    where
        F: FnOnce(bool) -> &'static str,
    {
        let bit = self.as_bit();
        let description = describer(bit).to_string();
        self.with_description(description)
    }

    fn describe<F>(self, describer: F) -> Result<Self, DecodeError>
    where
        F: FnOnce(u64) -> Result<&'static str, DecodeError>,
    {
        let description = describer(self.value)?.to_string();
        Ok(self.with_description(description))
    }

    fn check_res0(self) -> Result<Self, DecodeError> {
        if self.value != 0 {
            Err(DecodeError::InvalidRes0 { res0: self.value })
        } else {
            Ok(self)
        }
    }

    /// Returns the value as a hexadecimal string, or "true" or "false" if it is a single bit.
    pub fn value_string(&self) -> String {
        if self.width == 1 {
            if self.value == 1 { "true" } else { "false" }.to_string()
        } else {
            format!("{:#01$x}", self.value, (self.width + 3) / 4 + 2,)
        }
    }

    /// Returns the value as a binary strings.
    pub fn value_binary_string(&self) -> String {
        format!("{:#01$b}", self.value, self.width + 2)
    }
}

impl Display for FieldInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.width == 1 {
            write!(
                f,
                "{}: {}",
                self.name,
                if self.value == 1 { "true" } else { "false" }
            )
        } else {
            write!(
                f,
                "{}: {} {}",
                self.name,
                self.value_string(),
                self.value_binary_string(),
            )
        }
    }
}

/// An error decoding an ESR value.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// A RES0 field was not 0.
    #[error("Invalid ESR, res0 is {res0:#x}")]
    InvalidRes0 { res0: u64 },
    /// The EC field had an invalid value.
    #[error("Invalid EC {ec:#x}")]
    InvalidEc { ec: u64 },
    /// The DFSC or IFSC field had an invalid value.
    #[error("Invalid DFSC or IFSC {fsc:#x}")]
    InvalidFsc { fsc: u64 },
    /// The SET field had an invalid value.
    #[error("Invalid SET {set:#x}")]
    InvalidSet { set: u64 },
    /// The AET field had an invalid value.
    #[error("Invalid AET {aet:#x}")]
    InvalidAet { aet: u64 },
    /// The AM field had an invalid value.
    #[error("Invalid AM {am:#x}")]
    InvalidAm { am: u64 },
    /// The ISS field has an invalid value for a trapped LD64B or ST64B* exception.
    #[error("Invalid ISS {iss:#x} for trapped LD64B or ST64B*")]
    InvalidLd64bIss { iss: u64 },
}

fn decode_iss_res0(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 0, 25)
        .check_res0()?
        .with_description("ISS is RES0".to_string());
    Ok(vec![res0])
}

/// Decodes the given Exception Syndrome Register value, or returns an error if it is not valid.
pub fn decode(esr: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(esr, "RES0", Some("Reserved"), 37, 64).check_res0()?;
    let iss2 = FieldInfo::get(esr, "ISS2", None, 32, 37);
    let ec = FieldInfo::get(esr, "EC", Some("Exception Class"), 26, 32);
    let il =
        FieldInfo::get_bit(esr, "IL", Some("Instruction Length"), 25).describe_bit(describe_il);
    let iss = FieldInfo::get(esr, "ISS", Some("Instruction Specific Syndrome"), 0, 25);
    let (class, iss_subfields, iss_description) = match ec.value {
        0b000000 => ("Unknown reason", decode_iss_res0(iss.value)?, None),
        0b000001 => (
            "Wrapped WF* instruction execution",
            decode_iss_wf(iss.value)?,
            None,
        ),
        0b000011 => (
            "Trapped MCR or MRC access with coproc=0b1111",
            decode_iss_mcr(iss.value)?,
            None,
        ),
        0b000100 => (
            "Trapped MCRR or MRRC access with coproc=0b1111",
            decode_iss_mcrr(iss.value)?,
            None,
        ),
        0b000101 => (
            "Trapped MCR or MRC access with coproc=0b1110",
            decode_iss_mcr(iss.value)?,
            None,
        ),
        0b000110 => (
            "Trapped LDC or STC access",
            decode_iss_ldc(iss.value)?,
            None,
        ),
        0b000111 => (
            "Trapped access to SVE, Advanced SIMD or floating point",
            decode_iss_sve(iss.value)?,
            None,
        ),
        0b001010 => (
            "Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction",
            decode_iss_ld64b(iss.value)?,
            None,
        ),
        0b001100 => (
            "Trapped MRRC access with (coproc==0b1110)",
            decode_iss_mcrr(iss.value)?,
            None,
        ),
        0b001101 => ("Branch Target Exception", decode_iss_bti(iss.value)?, None),
        0b001110 => ("Illegal Execution state", decode_iss_res0(iss.value)?, None),
        0b010001 => (
            "SVC instruction execution in AArch32 state",
            decode_iss_hvc(iss.value)?,
            None,
        ),
        0b010101 => (
            "SVC instruction execution in AArch64 state",
            decode_iss_hvc(iss.value)?,
            None,
        ),
        0b011000 => {
            let (subfields, description) = decode_iss_msr(iss.value)?;
            (
                "Trapped MSR, MRS or System instruction execution in AArch64 state",
                subfields,
                description,
            )
        }
        0b011001 => (
            "Access to SVE functionality trapped as a result of CPACR_EL1.ZEN, CPTR_EL2.ZEN, \
             CPTR_EL2.TZ, or CPTR_EL3.EZ",
            decode_iss_res0(iss.value)?,
            None,
        ),
        0b011100 => (
            "Exception from a Pointer Authentication instruction authentication failure",
            decode_iss_pauth(iss.value)?,
            None,
        ),
        0b100000 => (
            "Instruction Abort from a lower Exception level",
            decode_iss_instruction_abort(iss.value)?,
            None,
        ),
        0b100001 => (
            "Instruction Abort taken without a change in Exception level",
            decode_iss_instruction_abort(iss.value)?,
            None,
        ),
        0b100010 => (
            "PC alignment fault exception",
            decode_iss_res0(iss.value)?,
            None,
        ),
        0b100100 => (
            "Data Abort from a lower Exception level",
            decode_iss_data_abort(iss.value)?,
            None,
        ),
        0b100101 => (
            "Data Abort taken without a change in Exception level",
            decode_iss_data_abort(iss.value)?,
            None,
        ),
        0b100110 => (
            "SP alignment fault exception",
            decode_iss_res0(iss.value)?,
            None,
        ),
        0b101000 => (
            "Trapped floating-point exception taken from AArch32 state",
            decode_iss_fp(iss.value)?,
            None,
        ),
        0b101100 => (
            "Trapped floating-point exception taken from AArch64 state",
            decode_iss_fp(iss.value)?,
            None,
        ),
        0b101111 => ("SError interrupt", decode_iss_serror(iss.value)?, None),
        0b110000 => (
            "Breakpoint exception from a lower Exception level",
            decode_iss_breakpoint_vector_catch(iss.value)?,
            None,
        ),
        0b110001 => (
            "Breakpoint exception taken without a change in Exception level",
            decode_iss_breakpoint_vector_catch(iss.value)?,
            None,
        ),
        0b110010 => (
            "Software Step exception from a lower Exception level",
            decode_iss_software_step(iss.value)?,
            None,
        ),
        0b110011 => (
            "Software Step exception taken without a change in Exception level",
            decode_iss_software_step(iss.value)?,
            None,
        ),
        0b110100 => (
            "Watchpoint exception from a lower Exception level",
            decode_iss_watchpoint(iss.value)?,
            None,
        ),
        0b110101 => (
            "Watchpoint exception taken without a change in Exception level",
            decode_iss_watchpoint(iss.value)?,
            None,
        ),
        0b111000 => (
            "BKPT instruction execution in AArch32 state",
            decode_iss_breakpoint(iss.value)?,
            None,
        ),
        0b111100 => (
            "BRK instruction execution in AArch64 state",
            decode_iss_breakpoint(iss.value)?,
            None,
        ),
        _ => return Err(DecodeError::InvalidEc { ec: ec.value }),
    };
    let iss = FieldInfo {
        description: iss_description,
        subfields: iss_subfields,
        ..iss
    };
    let ec = ec.with_description(class.to_string());
    Ok(vec![res0, iss2, ec, il, iss])
}

fn describe_il(il: bool) -> &'static str {
    if il {
        "32-bit instruction trapped"
    } else {
        "16-bit instruction trapped"
    }
}

/// Parses a decimal or hexadecimal number from a string.
///
/// If the string starts with `"0x"` then it will be parsed as hexadecimal, otherwise it will be
/// assumed to be decimal.
pub fn parse_number(s: &str) -> Result<u64, ParseIntError> {
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16)
    } else {
        s.parse()
    }
}
