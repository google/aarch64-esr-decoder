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

/// Decodes the ISS value for a Pointer Authentication failure.
pub fn decode_iss_pauth(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 2, 25).check_res0()?;
    let instruction_or_data =
        FieldInfo::get_bit(iss, "IorD", Some("Instruction key or Data key"), 1)
            .describe_bit(describe_instruction_or_data);
    let a_or_b =
        FieldInfo::get_bit(iss, "AorB", Some("A key or B key"), 0).describe_bit(describe_a_or_b);

    Ok(vec![res0, instruction_or_data, a_or_b])
}

fn describe_instruction_or_data(instruction_or_data: bool) -> &'static str {
    if instruction_or_data {
        "Data Key"
    } else {
        "Instruction Key"
    }
}

fn describe_a_or_b(a_or_b: bool) -> &'static str {
    if a_or_b {
        "B Key"
    } else {
        "A Key"
    }
}
