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

/// Decodes the ISS value for a trapped WF* instruction.
pub fn decode_iss_wf(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let cv =
        FieldInfo::get_bit(iss, "CV", Some("Condition code valid"), 24).describe_bit(describe_cv);
    let cond = FieldInfo::get(
        iss,
        "COND",
        Some("Condition code of the trapped instruction"),
        20,
        24,
    );
    let res0a = FieldInfo::get(iss, "RES0", Some("Reserved"), 10, 20).check_res0()?;
    let rn = FieldInfo::get(iss, "RN", Some("Register Number"), 5, 10);
    let res0b = FieldInfo::get(iss, "RES0", Some("Reserved"), 3, 5).check_res0()?;
    let rv = FieldInfo::get_bit(iss, "RV", Some("Register Valid"), 2).describe_bit(describe_rv);
    let ti = FieldInfo::get(iss, "TI", Some("Trapped Instruction"), 0, 2).describe(describe_ti)?;

    Ok(vec![cv, cond, res0a, rn, res0b, rv, ti])
}

fn describe_rv(rv: bool) -> &'static str {
    if rv {
        "RN is valid"
    } else {
        "RN is not valid"
    }
}

fn describe_ti(ti: u64) -> Result<&'static str, DecodeError> {
    Ok(match ti {
        0b00 => "WFI trapped",
        0b01 => "WFE trapped",
        0b10 => "WFIT trapped",
        0b11 => "WFET trapped",
        _ => unreachable!(),
    })
}
