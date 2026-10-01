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

use crate::{DecodeError, FieldInfo};
use std::fmt::{self, Debug, Display, Formatter};

/// Decodes the ISS value for an Instruction Abort.
pub fn decode_iss_instruction_abort(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0a = FieldInfo::get(iss, "RES0", Some("Reserved"), 15, 25).check_res0()?;
    let fnv = FieldInfo::get_bit(iss, "FnV", Some("FAR not Valid"), 10).describe_bit(describe_fnv);
    let ea = FieldInfo::get_bit(iss, "EA", Some("External abort type"), 9);
    let res0b = FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 8).check_res0()?;
    let s1ptw = FieldInfo::get_bit(iss, "S1PTW", Some("Stage-1 translation table walk"), 7);
    let res0c = FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 6).check_res0()?;
    let ifsc = FieldInfo::get(iss, "IFSC", Some("Instruction Fault Status Code"), 0, 6)
        .describe(describe_fsc)?;

    // PFV is only defined for synchronous External aborts (FEAT_PFAR).
    let pfv = if is_external_abort_fsc(ifsc.value) {
        FieldInfo::get_bit(iss, "PFV", Some("PFAR Valid"), 14).describe_bit(describe_pfv)
    } else {
        FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 14).check_res0()?
    };
    let res0d = FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 13).check_res0()?;

    let set = if ifsc.value == 0b010000 {
        FieldInfo::get(iss, "SET", Some("Synchronous Error Type"), 11, 13).describe(describe_set)?
    } else {
        FieldInfo::get(iss, "RES0", Some("Reserved"), 11, 13)
    };

    Ok(vec![
        res0a, pfv, res0d, set, fnv, ea, res0b, s1ptw, res0c, ifsc,
    ])
}

/// Decodes the ISS value for a Data Abort.
pub fn decode_iss_data_abort(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let isv = FieldInfo::get_bit(iss, "ISV", Some("Instruction Syndrome Valid"), 24)
        .describe_bit(describe_isv);

    let instruction_syndrome_fields = if isv.as_bit() {
        // These fields are part of the instruction syndrome, and are only valid if ISV is true.
        let sas = FieldInfo::get(iss, "SAS", Some("Syndrome Access Size"), 22, 24);
        let sas_value = match sas.value {
            0b00 => SyndromeAccessSize::Byte,
            0b01 => SyndromeAccessSize::Halfword,
            0b10 => SyndromeAccessSize::Word,
            0b11 => SyndromeAccessSize::Doubleword,
            _ => unreachable!(),
        };
        let sas = sas.with_description(sas_value.to_string());
        let sse = FieldInfo::get_bit(iss, "SSE", Some("Syndrome Sign Extend"), 21);
        let srt = FieldInfo::get(iss, "SRT", Some("Syndrome Register Transfer"), 16, 21);
        let sf = FieldInfo::get_bit(iss, "SF", Some("Sixty-Four"), 15).describe_bit(describe_sf);
        let ar =
            FieldInfo::get_bit(iss, "AR", Some("Acquire/Release"), 14).describe_bit(describe_ar);
        vec![sas, sse, srt, sf, ar]
    } else {
        // When ISV is 0, some of these bits have other meanings for certain fault types.
        let external_abort = is_external_abort_fsc(iss & 0b111111);
        let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 21, 24).check_res0()?;
        let mut fields = vec![res0];
        if external_abort {
            fields.push(FieldInfo::get(iss, "RES0", Some("Reserved"), 18, 21).check_res0()?);
            fields.push(
                FieldInfo::get(iss, "WU", Some("Write Update"), 16, 18).describe(describe_wu)?,
            );
        } else {
            fields.push(FieldInfo::get(iss, "RES0", Some("Reserved"), 16, 21).check_res0()?);
        }
        // FnP only describes the precision of FAR, so it is meaningless if FnV says that FAR is
        // not valid. FnV is only valid for DFSC 0b010000.
        let far_not_valid = iss & 0b111111 == 0b010000 && iss & (1 << 10) != 0;
        fields.push(
            FieldInfo::get_bit(iss, "FnP", Some("FAR not Precise"), 15)
                .describe_bit(|fnp| describe_fnp(fnp, far_not_valid)),
        );
        if external_abort {
            fields.push(
                FieldInfo::get_bit(iss, "PFV", Some("PFAR Valid"), 14).describe_bit(describe_pfv),
            );
        } else {
            fields.push(FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 14).check_res0()?);
        }
        fields
    };

    let vncr = FieldInfo::get_bit(iss, "VNCR", None, 13);
    let fnv = FieldInfo::get_bit(iss, "FnV", Some("FAR not Valid"), 10).describe_bit(describe_fnv);
    let ea = FieldInfo::get_bit(iss, "EA", Some("External abort type"), 9);
    let cm = FieldInfo::get_bit(iss, "CM", Some("Cache Maintenance"), 8);
    let s1ptw = FieldInfo::get_bit(iss, "S1PTW", Some("Stage-1 translation table walk"), 7);
    let wnr = FieldInfo::get_bit(iss, "WnR", Some("Write not Read"), 6).describe_bit(describe_wnr);
    let dfsc =
        FieldInfo::get(iss, "DFSC", Some("Data Fault Status Code"), 0, 6).describe(describe_fsc)?;
    let set = if dfsc.value == 0b010000 {
        FieldInfo::get(iss, "SET", Some("Synchronous Error Type"), 11, 13).describe(describe_set)?
    } else {
        FieldInfo::get(iss, "RES0", Some("Reserved"), 11, 13)
    };

    let mut fields = vec![isv];
    fields.extend(instruction_syndrome_fields);
    fields.extend(vec![vncr, set, fnv, ea, cm, s1ptw, wnr, dfsc]);
    Ok(fields)
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
        write!(f, "{s}")
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

/// Returns whether the given fault status code indicates a synchronous External abort, i.e. one
/// of `0b010000`, `0b01001x` or `0b0101xx`.
fn is_external_abort_fsc(fsc: u64) -> bool {
    fsc == 0b010000 || fsc & 0b111110 == 0b010010 || fsc & 0b111100 == 0b010100
}

pub(super) fn describe_pfv(pfv: bool) -> &'static str {
    if pfv {
        "MFAR_ELx is valid"
    } else {
        "MFAR_ELx is UNKNOWN"
    }
}

fn describe_fnp(fnp: bool, far_not_valid: bool) -> &'static str {
    if far_not_valid {
        "Not applicable, FAR is not valid (see FnV)"
    } else if fnp {
        "FAR holds any virtual address within the naturally-aligned granule containing the \
         faulting address"
    } else {
        "FAR holds the faulting virtual address"
    }
}

pub(super) fn describe_wu(wu: u64) -> Result<&'static str, DecodeError> {
    Ok(match wu {
        0b00 => {
            "Not a store instruction or translation table update, or the location might have \
             been updated"
        }
        0b10 => "Store instruction or translation table update that did not update the location",
        0b11 => "Store instruction or translation table update that updated the location",
        _ => "Reserved",
    })
}

fn describe_fnv(fnv: bool) -> &'static str {
    if fnv {
        "FAR is not valid, it holds an unknown value"
    } else {
        "FAR is valid"
    }
}

pub(super) fn describe_wnr(wnr: bool) -> &'static str {
    if wnr {
        "Abort caused by writing to memory"
    } else {
        "Abort caused by reading from memory"
    }
}

fn describe_fsc(fsc: u64) -> Result<&'static str, DecodeError> {
    let description = match fsc {
        0b000000 => {
            "Address size fault, level 0 of translation or translation table base register."
        }
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
        0b010000 => {
            "Synchronous External abort, not on translation table walk or hardware update of \
             translation table."
        }
        0b010001 => "Synchronous Tag Check Fault.",
        0b010011 => {
            "Synchronous External abort on translation table walk or hardware update of \
             translation table, level -1."
        }
        0b010100 => {
            "Synchronous External abort on translation table walk or hardware update of \
             translation table, level 0."
        }
        0b010101 => {
            "Synchronous External abort on translation table walk or hardware update of \
             translation table, level 1."
        }
        0b010110 => {
            "Synchronous External abort on translation table walk or hardware update of \
             translation table, level 2."
        }
        0b010111 => {
            "Synchronous External abort on translation table walk or hardware update of \
             translation table, level 3."
        }
        0b011000 => {
            "Synchronous parity or ECC error on memory access, not on translation table walk."
        }
        0b011011 => {
            "Synchronous parity or ECC error on memory access on translation table walk or \
             hardware update of translation table, level -1."
        }
        0b011100 => {
            "Synchronous parity or ECC error on memory access on translation table walk or \
             hardware update of translation table, level 0."
        }
        0b011101 => {
            "Synchronous parity or ECC error on memory access on translation table walk or \
             hardware update of translation table, level 1."
        }
        0b011110 => {
            "Synchronous parity or ECC error on memory access on translation table walk or \
             hardware update of translation table, level 2."
        }
        0b011111 => {
            "Synchronous parity or ECC error on memory access on translation table walk or \
             hardware update of translation table, level 3."
        }
        0b100001 => "Alignment fault.",
        0b100011 => {
            "Granule Protection Fault on translation table walk or hardware update of \
            translation table, level -1."
        }
        0b100100 => {
            "Granule Protection Fault on translation table walk or hardware update of \
            translation table, level 0."
        }
        0b100101 => {
            "Granule Protection Fault on translation table walk or hardware update of \
            translation table, level 1."
        }
        0b100110 => {
            "Granule Protection Fault on translation table walk or hardware update of \
            translation table, level 2."
        }
        0b100111 => {
            "Granule Protection Fault on translation table walk or hardware update of \
            translation table, level 3."
        }
        0b101000 => {
            "Granule Protection Fault, not on translation table walk or hardware update of \
            translation table."
        }
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
