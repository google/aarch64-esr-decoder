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

use abort::{decode_iss_data_abort, decode_iss_instruction_abort};
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

    fn with_description(self, description: String) -> Self {
        self.with_decoded(Decoded {
            description: Some(description),
            fields: vec![],
        })
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
    #[error("Invalid ESR, res0 is {res0:#x}")]
    InvalidRes0 { res0: u64 },
    #[error("Invalid EC {ec:#x}")]
    InvalidEc { ec: u64 },
    #[error("Invalid DFSC or IFSC {fsc:#x}")]
    InvalidFsc { fsc: u64 },
    #[error("ISV was false but instruction syndrome bits were nonzero {is:#x}")]
    UnexpectedInstructionSyndrome { is: u64 },
    #[error("Invalid SET {set:#x}")]
    InvalidSet { set: u64 },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decoded {
    pub description: Option<String>,
    pub fields: Vec<FieldInfo>,
}

fn decode_iss_res0(iss: u64) -> Result<Decoded, DecodeError> {
    if iss == 0 {
        Ok(Decoded {
            description: Some("ISS is RES0".to_string()),
            fields: vec![],
        })
    } else {
        Err(DecodeError::InvalidRes0 { res0: iss })
    }
}

pub fn decode(esr: u64) -> Result<Decoded, DecodeError> {
    let res0 = FieldInfo::get(esr, "RES0", 37, 64).check_res0()?;
    let iss2 = FieldInfo::get(esr, "ISS2", 32, 37);
    let ec = FieldInfo::get(esr, "EC", 26, 32);
    let il = FieldInfo::get_bit(esr, "IL", 25);
    let iss = FieldInfo::get(esr, "ISS", 0, 25);
    let (class, iss_decoded) = match ec.value {
        0b000000 => ("Unknown reason", Some(decode_iss_res0(iss.value)?)),
        0b000001 => ("Wrapped WF* instruction execution", None),
        0b000011 => ("Trapped MCR or MRC access with coproc=0b1111", None),
        0b000100 => ("Trapped MCRR or MRRC access with coproc=0b1111", None),
        0b000101 => ("Trapped MCR or MRC access with coproc=0b1110", None),
        0b000110 => ("Trapped LDC or STC access", None),
        0b000111 => ("Trapped access to SVE, Advanced SIMD or floating point", None),
        0b001010 => ("Trapped execution of an LD64B, ST64B, ST64BV, or ST64BV0 instruction", None),
        0b001100 => ("Trapped MRRC access with (coproc==0b1110)", None),
        0b001101 => ("Branch Target Exception", None),
        0b001110 => ("Illegal Execution state", Some(decode_iss_res0(iss.value)?)),
        0b010001 => ("SVC instruction execution in AArch32 state", None),
        0b010101 => ("SVC instruction execution in AArch64 state", None),
        0b011000 => ("Trapped MSR, MRS or System instruction execution in AArch64 state", None),
        0b011001 => ("Access to SVE functionality trapped as a result of CPACR_EL1.ZEN, CPTR_EL2.ZEN, CPTR_EL2.TZ, or CPTR_EL3.EZ", Some(decode_iss_res0(iss.value)?)),
        0b011100 => ("Exception from a Pointer Authentication instruction authentication failure", None),
        0b100000 => ("Instruction Abort from a lower Exception level", Some(decode_iss_instruction_abort(iss.value)?)),
        0b100001 => ("Instruction Abort taken without a change in Exception level", Some(decode_iss_instruction_abort(iss.value)?)),
        0b100010 => ("PC alignment fault exception", Some(decode_iss_res0(iss.value)?)),
        0b100100 => ("Data Abort from a lower Exception level", Some(decode_iss_data_abort(iss.value)?)),
        0b100101 => ("Data Abort taken without a change in Exception level", Some(decode_iss_data_abort(iss.value)?)),
        0b100110 => ("SP alignment fault exception", Some(decode_iss_res0(iss.value)?)),
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
        description: Some(class.to_string()),
        fields: vec![],
    });
    Ok(Decoded {
        description: Some(class.to_string()),
        fields: vec![res0, iss2, ec, il, iss],
    })
}
