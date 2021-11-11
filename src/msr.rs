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

/// Decodes the ISS value for an MSR or MRS access.
pub fn decode_iss_msr(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", 22, 25).check_res0()?;
    let op0 = FieldInfo::get(iss, "Op0", 20, 22);
    let op2 = FieldInfo::get(iss, "Op2", 17, 20);
    let op1 = FieldInfo::get(iss, "Op1", 14, 17);
    let crn = FieldInfo::get(iss, "CRn", 10, 14);
    let rt = FieldInfo::get(iss, "Rt", 5, 10);
    let crm = FieldInfo::get(iss, "CRm", 1, 5);
    let direction = FieldInfo::get_bit(iss, "Direction", 0).describe_bit(describe_direction);

    Ok(vec![res0, op0, op2, op1, crn, rt, crm, direction])
}

fn describe_direction(direction: bool) -> &'static str {
    if direction {
        "Read from system register (MRS)"
    } else {
        "Write to system register (MSR)"
    }
}
