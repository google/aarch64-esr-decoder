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

/// Decodes the ISS value for a Breakpoint or Vector Catch debug exception.
pub fn decode_iss_breakpoint_vector_catch(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 6, 25).check_res0()?;
    let ifsc = FieldInfo::get(iss, "IFSC", Some("Instruction Fault Status Code"), 0, 6)
        .describe(describe_fsc)?;

    Ok(vec![res0, ifsc])
}

/// Decodes the ISS value for a Software Step exception.
pub fn decode_iss_software_step(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let isv = FieldInfo::get_bit(iss, "ISV", Some("Instruction Syndrome Valid"), 24)
        .describe_bit(describe_isv);
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 7, 24).check_res0()?;
    let ex = if isv.as_bit() {
        FieldInfo::get_bit(iss, "EX", Some("Exclusive operation"), 6).describe_bit(describe_ex)
    } else {
        FieldInfo::get_bit(iss, "RES0", Some("Reserved because ISV is false"), 6).check_res0()?
    };
    let ifsc = FieldInfo::get(iss, "IFSC", Some("Instruction Fault Status Code"), 0, 6)
        .describe(describe_fsc)?;

    Ok(vec![isv, res0, ex, ifsc])
}

/// Decodes the ISS value for a Watchpoint exception.
pub fn decode_iss_watchpoint(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0a = FieldInfo::get(iss, "RES0", Some("Reserved"), 15, 25).check_res0()?;
    let res0b = FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 14).check_res0()?;
    let vncr = FieldInfo::get_bit(iss, "VNCR", None, 13);
    let res0c = FieldInfo::get(iss, "RES0", Some("Reserved"), 9, 13).check_res0()?;
    let cm = FieldInfo::get_bit(iss, "CM", Some("Cache Maintenance"), 8);
    let res0d = FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 7).check_res0()?;
    let wnr = FieldInfo::get_bit(iss, "WnR", Some("Write not Read"), 6).describe_bit(describe_wnr);
    let dfsc =
        FieldInfo::get(iss, "DFSC", Some("Data Fault Status Code"), 0, 6).describe(describe_fsc)?;

    Ok(vec![res0a, res0b, vncr, res0c, cm, res0d, wnr, dfsc])
}

/// Decodes the ISS value for a Breakpoint instruction.
pub fn decode_iss_breakpoint(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 16, 25).check_res0()?;
    let comment = FieldInfo::get(
        iss,
        "Comment",
        Some("Instruction comment field or immediate field"),
        0,
        16,
    );

    Ok(vec![res0, comment])
}

fn describe_fsc(fsc: u64) -> Result<&'static str, DecodeError> {
    match fsc {
        0b100010 => Ok("Debug exception"),
        _ => Err(DecodeError::InvalidFsc { fsc }),
    }
}

fn describe_isv(isv: bool) -> &'static str {
    if isv {
        "EX bit is valid"
    } else {
        "EX bit is RES0"
    }
}

fn describe_ex(ex: bool) -> &'static str {
    if ex {
        "A Load-Exclusive instruction was stepped"
    } else {
        "Some instruction other than a Load-Exclusive was stepped"
    }
}

fn describe_wnr(wnr: bool) -> &'static str {
    if wnr {
        "Watchpoint caused by writing to memory"
    } else {
        "Watchpoint caused by reading from memory"
    }
}
