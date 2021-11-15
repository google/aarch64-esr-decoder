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

/// Decodes the ISS value for a trapped LD64B or ST64B* instruction.
pub fn decode_iss_ld64b(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let iss = FieldInfo::get(iss, "ISS", None, 0, 25).describe(describe_iss_ld64b)?;
    Ok(vec![iss])
}

fn describe_iss_ld64b(iss: u64) -> Result<&'static str, DecodeError> {
    match iss {
        0b00 => Ok("ST64BV trapped"),
        0b01 => Ok("ST64BV0 trapped"),
        0b10 => Ok("LD64B or ST64B trapped"),
        _ => Err(DecodeError::InvalidLd64bIss { iss }),
    }
}
