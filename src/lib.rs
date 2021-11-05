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

fn describe_isv(isv: bool) -> &'static str {
    if isv {
        "Valid instruction syndrome"
    } else {
        "No valid instruction syndrome"
    }
}

fn describe_sf(sf: bool) -> &'static str {
    if sf {
        "64-bit wide register"
    } else {
        "32-bit wide register"
    }
}

fn describe_ar(ar: bool) -> &'static str {
    if ar {
        "Acquire/release semantics"
    } else {
        "No acquire/release semantics"
    }
}

fn describe_fnv(fnv: bool) -> &'static str {
    if fnv {
        "FAR is not valid, it holds an unknown value"
    } else {
        "FAR is valid"
    }
}

fn describe_wnr(wnr: bool) -> &'static str {
    if wnr {
        "Abort caused by writing to memory"
    } else {
        "Abort caused by reading from memory"
    }
}

fn describe_fsc(fsc: u64) -> Result<&'static str, DecodeError> {
    let description = match fsc {
        0b000000 => "Address size fault, level 0 of translation or translation table base register.",
        0b000001 => "Address size fault, level 1.",
        0b000010 => "Address size fault, level 2.",
        0b000011 => "Address size fault, level 3.",
        0b000100 => "Translation fault, level 0.",
        0b000101 => "Translation fault, level 1.",
        0b000110 => "Translation fault, level 2.",
        0b000111 => "Translation fault, level 3.",
        0b001001 => "Access flag fault, level 1.",
        0b001010 => "Access flag fault, level 2.",
        0b001011 => "Access flag fault, level 3.",
        0b001000 => "Access flag fault, level 0.",
        0b001100 => "Permission fault, level 0.",
        0b001101 => "Permission fault, level 1.",
        0b001110 => "Permission fault, level 2.",
        0b001111 => "Permission fault, level 3.",
        0b010000 => "Synchronous External abort, not on translation table walk or hardware update of translation table.",
        0b010001 => "Synchronous Tag Check Fault.",
        0b010011 => "Synchronous External abort on translation table walk or hardware update of translation table, level -1.",
        0b010100 => "Synchronous External abort on translation table walk or hardware update of translation table, level 0.",
        0b010101 => "Synchronous External abort on translation table walk or hardware update of translation table, level 1.",
        0b010110 => "Synchronous External abort on translation table walk or hardware update of translation table, level 2.",
        0b010111 => "Synchronous External abort on translation table walk or hardware update of translation table, level 3.",
        0b011000 => "Synchronous parity or ECC error on memory access, not on translation table walk.",
        0b011011 => "Synchronous parity or ECC error on memory access on translation table walk or hardware update of translation table, level -1.",
        0b011100 => "Synchronous parity or ECC error on memory access on translation table walk or hardware update of translation table, level 0.",
        0b011101 => "Synchronous parity or ECC error on memory access on translation table walk or hardware update of translation table, level 1.",
        0b011110 => "Synchronous parity or ECC error on memory access on translation table walk or hardware update of translation table, level 2.",
        0b011111 => "Synchronous parity or ECC error on memory access on translation table walk or hardware update of translation table, level 3.",
        0b100001 => "Alignment fault.",
        0b101001 => "Address size fault, level -1.",
        0b101011 => "Translation fault, level -1.",
        0b110000 => "TLB conflict abort.",
        0b110001 => "Unsupported atomic hardware update fault.",
        0b110100 => "IMPLEMENTATION DEFINED fault (Lockdown).",
        0b110101 => "IMPLEMENTATION DEFINED fault (Unsupported Exclusive or Atomic access).",
        _ => return Err(DecodeError::InvalidFsc { fsc }),
    };
    Ok(description)
}

fn describe_set(set: u64) -> Result<&'static str, DecodeError> {
    Ok(match set {
        0b00 => "Recoverable state (UER)",
        0b10 => "Uncontainable (UC)",
        0b11 => "Restartable state (UEO)",
        _ => return Err(DecodeError::InvalidSet { set }),
    })
}

fn decode_iss_instruction_abort(iss: u64) -> Result<Decoded, DecodeError> {
    let res0a = FieldInfo::get(iss, "RES0", 13, 25).check_res0()?;
    let fnv = FieldInfo::get_bit(iss, "FnV", 10).describe_bit(describe_fnv);
    let ea = FieldInfo::get_bit(iss, "EA", 9);
    let res0b = FieldInfo::get_bit(iss, "RES0", 8).check_res0()?;
    let s1ptw = FieldInfo::get_bit(iss, "S1PTW", 7);
    let res0c = FieldInfo::get_bit(iss, "RES0", 6).check_res0()?;
    let ifsc = FieldInfo::get(iss, "IFSC", 0, 6).describe(describe_fsc)?;

    let set = if ifsc.value == 0b010000 {
        FieldInfo::get(iss, "SET", 11, 13).describe(describe_set)?
    } else {
        FieldInfo::get(iss, "RES0", 11, 13)
    };

    Ok(Decoded {
        description: None,
        fields: vec![res0a, set, fnv, ea, res0b, s1ptw, res0c, ifsc],
    })
}

fn decode_iss_data_abort(iss: u64) -> Result<Decoded, DecodeError> {
    let isv = FieldInfo::get_bit(iss, "ISV", 24).describe_bit(describe_isv);

    let intruction_syndrome_fields = if isv.as_bit() {
        // These fields are part of the instruction syndrome, and are only valid if ISV is true.
        let sas = FieldInfo::get(iss, "SAS", 22, 24);
        let sas_value = match sas.value {
            0b00 => SyndromeAccessSize::Byte,
            0b01 => SyndromeAccessSize::Halfword,
            0b10 => SyndromeAccessSize::Word,
            0b11 => SyndromeAccessSize::Doubleword,
            _ => unreachable!(),
        };
        let sas = sas.with_decoded(Decoded {
            description: Some(sas_value.to_string()),
            fields: vec![],
        });
        let sse = FieldInfo::get_bit(iss, "SSE", 21);
        let srt = FieldInfo::get(iss, "SRT", 16, 21);
        let sf = FieldInfo::get_bit(iss, "SF", 15).describe_bit(describe_sf);
        let ar = FieldInfo::get_bit(iss, "AR", 14).describe_bit(describe_ar);
        vec![sas, sse, srt, sf, ar]
    } else {
        let res0 = FieldInfo::get(iss, "RES0", 14, 24);
        if res0.value != 0 {
            return Err(DecodeError::UnexpectedInstructionSyndrome { is: res0.value });
        }
        vec![res0]
    };

    let vncr = FieldInfo::get_bit(iss, "VNCR", 13);
    let fnv = FieldInfo::get_bit(iss, "FnV", 10).describe_bit(describe_fnv);
    let ea = FieldInfo::get_bit(iss, "EA", 9);
    let cm = FieldInfo::get_bit(iss, "CM", 8);
    let s1ptw = FieldInfo::get_bit(iss, "S1PTW", 7);
    let wnr = FieldInfo::get_bit(iss, "WnR", 6).describe_bit(describe_wnr);
    let dfsc = FieldInfo::get(iss, "DFSC", 0, 6).describe(describe_fsc)?;

    let mut fields = vec![isv];
    fields.extend(intruction_syndrome_fields);
    fields.extend(vec![vncr, fnv, ea, cm, s1ptw, wnr, dfsc]);
    Ok(Decoded {
        description: None,
        fields,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Decoded {
    pub description: Option<String>,
    pub fields: Vec<FieldInfo>,
}

pub fn decode(esr: u64) -> Result<Decoded, DecodeError> {
    let res0 = FieldInfo::get(esr, "RES0", 37, 64).check_res0()?;
    let iss2 = FieldInfo::get(esr, "ISS2", 32, 37);
    let ec = FieldInfo::get(esr, "EC", 26, 32);
    let il = FieldInfo::get_bit(esr, "IL", 25);
    let iss = FieldInfo::get(esr, "ISS", 0, 25);
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
        0b100000 => ("Instruction Abort from a lower Exception level", Some(decode_iss_instruction_abort(iss.value)?)),
        0b100001 => ("Instruction Abort taken without a change in Exception level", Some(decode_iss_instruction_abort(iss.value)?)),
        0b100010 => ("PC alignment fault exception", None),
        0b100100 => ("Data Abort from a lower Exception level", Some(decode_iss_data_abort(iss.value)?)),
        0b100101 => ("Data Abort taken without a change in Exception level", Some(decode_iss_data_abort(iss.value)?)),
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
        description: Some(class.to_string()),
        fields: vec![],
    });
    Ok(Decoded {
        description: Some(class.to_string()),
        fields: vec![res0, iss2, ec, il, iss],
    })
}
