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

use super::common::describe_cv;
use crate::{DecodeError, FieldInfo};

/// Decodes the ISS value for a trapped LDC or STC instruction.
pub fn decode_iss_ldc(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let cv =
        FieldInfo::get_bit(iss, "CV", Some("Condition code valid"), 24).describe_bit(describe_cv);
    let cond = FieldInfo::get(
        iss,
        "COND",
        Some("Condition code of the trapped instruction"),
        20,
        24,
    );
    let imm8 = FieldInfo::get(
        iss,
        "imm8",
        Some("Immediate value of the trapped instruction"),
        12,
        20,
    );
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 10, 12).check_res0()?;
    let rn = FieldInfo::get(
        iss,
        "Rn",
        Some("General-purpose register number of the trapped instruction"),
        5,
        10,
    );
    let offset = FieldInfo::get_bit(
        iss,
        "Offset",
        Some("Whether the offset is added or subtracted"),
        4,
    )
    .describe_bit(describe_offset);
    let am = FieldInfo::get(iss, "AM", Some("Addressing Mode"), 1, 4).describe(describe_am)?;
    let direction = FieldInfo::get_bit(
        iss,
        "Direction",
        Some("Direction of the trapped instruction"),
        0,
    )
    .describe_bit(describe_direction);

    Ok(vec![cv, cond, imm8, res0, rn, offset, am, direction])
}

fn describe_offset(offset: bool) -> &'static str {
    if offset {
        "Add offset"
    } else {
        "Subtract offset"
    }
}

fn describe_am(am: u64) -> Result<&'static str, DecodeError> {
    match am {
        0b000 => Ok("Immediate unindexed"),
        0b001 => Ok("Immediate post-indexed"),
        0b010 => Ok("Immediate offset"),
        0b011 => Ok("Immediate pre-indexed"),
        0b100 => Ok("Reserved for trapped STR or T32 LDC"),
        0b110 => Ok("Reserved for trapped STC"),
        _ => Err(DecodeError::InvalidAm { am }),
    }
}

fn describe_direction(direction: bool) -> &'static str {
    if direction {
        "Read from memory (LDC)"
    } else {
        "Write to memory (STC)"
    }
}
