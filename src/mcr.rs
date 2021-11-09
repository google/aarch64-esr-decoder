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

use crate::common::describe_cv;
use crate::{DecodeError, Decoded, FieldInfo};

/// Decodes the ISS value for an MCR or MRC access.
pub fn decode_iss_mcr(iss: u64) -> Result<Decoded, DecodeError> {
    let cv = FieldInfo::get_bit(iss, "CV", 24).describe_bit(describe_cv);
    let cond = FieldInfo::get(iss, "COND", 20, 24);
    let opc2 = FieldInfo::get(iss, "Opc2", 17, 20);
    let opc1 = FieldInfo::get(iss, "Opc1", 14, 17);
    let rt = FieldInfo::get(iss, "Rt", 5, 10);
    let crm = FieldInfo::get(iss, "CRm", 1, 5);
    let direction = FieldInfo::get_bit(iss, "Direction", 0).describe_bit(describe_direction);

    Ok(Decoded {
        description: None,
        fields: vec![cv, cond, opc2, opc1, rt, crm, direction],
    })
}

/// Decodes the ISS value for an MCRR or MRRC access.
pub fn decode_iss_mcrr(iss: u64) -> Result<Decoded, DecodeError> {
    let cv = FieldInfo::get_bit(iss, "CV", 24).describe_bit(describe_cv);
    let cond = FieldInfo::get(iss, "COND", 20, 24);
    let opc1 = FieldInfo::get(iss, "Opc2", 16, 20);
    let res0 = FieldInfo::get_bit(iss, "RES0", 15).check_res0()?;
    let rt2 = FieldInfo::get(iss, "Rt2", 10, 15);
    let rt = FieldInfo::get(iss, "Rt", 5, 10);
    let crm = FieldInfo::get(iss, "CRm", 1, 5);
    let direction = FieldInfo::get_bit(iss, "Direction", 0).describe_bit(describe_direction);

    Ok(Decoded {
        description: None,
        fields: vec![cv, cond, opc1, res0, rt2, rt, crm, direction],
    })
}

fn describe_direction(direction: bool) -> &'static str {
    if direction {
        "Read from system register (MRC or VMRS)"
    } else {
        "Write to system register (MCR)"
    }
}