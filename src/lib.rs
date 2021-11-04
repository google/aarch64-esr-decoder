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

use bit_field::BitField;
use std::fmt::{self, Debug, Display, Formatter};
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldInfo {
    pub name: &'static str,
    pub start: usize,
    pub width: usize,
    pub value: u64,
}

impl FieldInfo {
    fn get(esr: u64, name: &'static str, start: usize, end: usize) -> Self {
        let value = esr.get_bits(start..end);
        Self {
            name,
            start,
            width: end - start,
            value,
        }
    }

    fn get_bit(register: u64, name: &'static str, bit: usize) -> Self {
        Self::get(register, name, bit, bit + 1)
    }
}

impl Display for FieldInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.width == 1 {
            write!(
                f,
                "{}:{}",
                self.name,
                if self.value == 1 { "true" } else { "false" }
            )
        } else {
            write!(f, "{}:{:#x}", self.name, self.value)
        }
    }
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Invalid ESR, res0 is {res0}")]
    InvalidRes0 { res0: u64 },
    #[error("Invalid EC {ec}")]
    InvalidEc { ec: u64 },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SyndromeAccessSize {
    Byte = 0b00,
    Halfword = 0b01,
    Word = 0b10,
    Doubleword = 0b11,
}

impl Display for SyndromeAccessSize {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let s = match self {
            Self::Byte => "byte",
            Self::Halfword => "halfword",
            Self::Word => "word",
            Self::Doubleword => "doubleword",
        };
        write!(f, "{}", s)
    }
}

fn decode_iss_data_abort(iss: u64) -> String {
    let isv = iss.get_bit(24);
    let sas = match iss.get_bits(22..24) {
        0b00 => SyndromeAccessSize::Byte,
        0b01 => SyndromeAccessSize::Halfword,
        0b10 => SyndromeAccessSize::Word,
        0b11 => SyndromeAccessSize::Doubleword,
        _ => unreachable!(),
    };
    let sse = iss.get_bit(21);
    let src = iss.get_bits(16..21);
    let sf = iss.get_bit(15);
    let ar = iss.get_bit(14);
    let vncr = iss.get_bit(13);
    let fnv = iss.get_bit(10);
    let ea = iss.get_bit(9);
    let cm = iss.get_bit(8);
    let s1ptw = iss.get_bit(7);
    let wnr = iss.get_bit(6);
    let dfsc = iss.get_bits(0..6);
    format!("ISV:{}, SAS:{}", isv, sas)
}

pub struct Decoded {
    pub fields: Vec<FieldInfo>,
    pub description: String,
}

pub fn decode(esr: u64) -> Result<Decoded, DecodeError> {
    let res0 = FieldInfo::get(esr, "RES0", 37, 64);
    let iss2 = FieldInfo::get(esr, "ISS2", 32, 37);
    let ec = FieldInfo::get(esr, "EC", 26, 32);
    let il = FieldInfo::get_bit(esr, "IL", 25);
    let iss = FieldInfo::get(esr, "ISS", 0, 25);
    if res0.value != 0 {
        return Err(DecodeError::InvalidRes0 { res0: res0.value });
    }
    let (class, iss_description) = match ec.value {
        0b000000 => ("Unknown reason", None),
        0b000001 => ("Wrapped WF* instruction execution", None),
        0b000011 => ("Trapped MCR or MRC access with coproc=0b1111", None),
        0b000100 => ("Trapped MCRR or MRRC access with coproc=0b1111", None),
        0b000101 => ("Trapped MCR or MRC access with coproc=0b1110", None),
        0b000110 => ("Trapped LDC or STC access", None),
        0b000111 => ("Trapped access to SVE, Advanced SIMD or floating point", None),
        0b001010 => ("Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction", None),
        0b001100 => ("Trapped MRRC access with (coproc==0b1110)", None),
        0b001101 => ("Branch Target Exception", None),
        0b001110 => ("Illegal Execution state", None),
        0b010001 => ("SVC instruction execution in AArch32 state", None),
        0b010101 => ("SVC instruction execution in AArch64 state", None),
        0b011000 => ("Trapped MSR, MRS or System instruction execution in AArch64 state", None),
        0b011001 => ("Access to SVE functionality trapped as a result of CPACR_EL1.ZEN, CPTR_EL2.ZEN, CPTR_EL2.TZ, or CPTR_EL3.EZ", None),
        0b011100 => ("Exception from a Pointer Authentication instruction authentication failure", None),
        0b100000 => ("Instruction Abort from a lower Exception level", None),
        0b100001 => ("Instruction Abort taken without a change in Exception level", None),
        0b100010 => ("PC alignment fault exception", None),
        0b100100 => ("Data Abort from a lower Exception level", Some(decode_iss_data_abort(iss.value))),
        0b100101 => ("Data Abort taken without a change in Exception level", Some(decode_iss_data_abort(iss.value))),
        0b100110 => ("SP alignment fault exception", None),
        0b101000 => ("Trapped floating-point exception taken from AArch32 state", None),
        0b101100 => ("Trapped floating-point exception taken from AArch64 state", None),
        0b101111 => ("SError interrupt", None),
        0b110000 => ("Breakpoint exception from a lower Exception level", None),
        0b110001 => ("Breakpoint exception taken without a change in Exception level", None),
        0b110010 => ("Software Step exception from a lower Exception level", None),
        0b110011 => ("Software Step exception taken without a change in Exception level", None),
        0b110100 => ("Watchpoint exception from a lower Exception level", None),
        0b110101 => ("Watchpoint exception taken without a change in Exception level", None),
        0b111000 => ("BKPT instruction execution in AArch32 state", None),
        0b111100 => ("BRK instruction execution in AArch64 state", None),
        _ => return Err(DecodeError::InvalidEc { ec: ec.value }),
    };
    let description = if let Some(iss_description) = iss_description {
        format!(
            "EC:{:#08b} '{}', {}, {} ({}), {}",
            ec.value, class, il, iss, iss_description, iss2
        )
    } else {
        format!(
            "EC:{:#08b} '{}', {}, {}, {}",
            ec.value, class, il, iss, iss2
        )
    };
    Ok(Decoded {
        fields: vec![res0, iss2, ec, il, iss],
        description,
    })
}
