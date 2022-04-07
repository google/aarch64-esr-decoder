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

/// Decodes the ISS value for a trapped SVE, Advanced SIMD or FP instruction.
pub fn decode_iss_sve(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let cv =
        FieldInfo::get_bit(iss, "CV", Some("Condition code valid"), 24).describe_bit(describe_cv);
    let cond = FieldInfo::get(
        iss,
        "COND",
        Some("Condition code of the trapped instruction"),
        20,
        24,
    );
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 0, 20).check_res0()?;

    Ok(vec![cv, cond, res0])
}
