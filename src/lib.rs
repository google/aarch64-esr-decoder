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
    pub decoded: Option<Decoded>,
}

impl FieldInfo {
    fn get(register: u64, name: &'static str, start: usize, end: usize) -> Self {
        let value = register.get_bits(start..end);
        Self {
            name,
            start,
            width: end - start,
            value,
            decoded: None,
        }
    }

    fn get_bit(register: u64, name: &'static str, bit: usize) -> Self {
        Self::get(register, name, bit, bit + 1)
    }

    fn with_decoded(self, decoded: Decoded) -> Self {
        Self {
            decoded: Some(decoded),
            ..self
        }
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
                "{}: {:#02$x} {1:#03$b}",
                self.name,
                self.value,
                (self.width + 3) / 4 + 2,
                self.width + 2
            )
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

fn decode_iss_data_abort(iss: u64) -> Decoded {
    let isv = FieldInfo::get_bit(iss, "ISV", 24);
    let sas = FieldInfo::get(iss, "SAS", 22, 24);
    let sas_decoded = match sas.value {
        0b00 => SyndromeAccessSize::Byte,
        0b01 => SyndromeAccessSize::Halfword,
        0b10 => SyndromeAccessSize::Word,
        0b11 => SyndromeAccessSize::Doubleword,
        _ => unreachable!(),
    };
    let sse = FieldInfo::get_bit(iss, "SSE", 21);
    let srt = FieldInfo::get(iss, "SRT", 16, 21);
    let sf = FieldInfo::get_bit(iss, "SF", 15);
    let ar = FieldInfo::get_bit(iss, "AR", 14);
    let vncr = FieldInfo::get_bit(iss, "VNCR", 13);
    let fnv = FieldInfo::get_bit(iss, "FnV", 10);
    let ea = FieldInfo::get_bit(iss, "EA", 9);
    let cm = FieldInfo::get_bit(iss, "CM", 8);
    let s1ptw = FieldInfo::get_bit(iss, "S1PTW", 7);
    let wnr = FieldInfo::get_bit(iss, "WnR", 6);
    let dfsc = FieldInfo::get(iss, "DFSC", 0, 6);
    let description = format!("{}, SAS:{}", isv, sas_decoded);
    Decoded {
        description,
        fields: vec![
            isv, sas, sse, srt, sf, ar, vncr, fnv, ea, cm, s1ptw, wnr, dfsc,
        ],
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decoded {
    pub description: String,
    pub fields: Vec<FieldInfo>,
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
    let (class, iss_decoded) = match ec.value {
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
    let iss = FieldInfo {
        decoded: iss_decoded,
        ..iss
    };
    let ec = ec.with_decoded(Decoded {
        description: class.to_string(),
        fields: vec![],
    });
    Ok(Decoded {
        description: class.to_string(),
        fields: vec![res0, iss2, ec, il, iss],
    })
}
