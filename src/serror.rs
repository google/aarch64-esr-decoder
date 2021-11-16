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

/// Decodes the ISS value for an SError interrupt.
pub fn decode_iss_serror(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let ids = FieldInfo::get_bit(iss, "IDS", Some("Implementation Defined Syndrome"), 24)
        .describe_bit(describe_ids);
    let platform_fields = if ids.as_bit() {
        let impdef = FieldInfo::get(iss, "IMPDEF", Some("Implementation defined"), 0, 24);
        vec![impdef]
    } else {
        let dfsc = FieldInfo::get(iss, "DFSC", Some("Data Fault Status Code"), 0, 6)
            .describe(describe_dfsc)?;

        let res0a = FieldInfo::get(iss, "RES0", Some("Reserved"), 14, 24).check_res0()?;
        let iesb = if dfsc.value == 0b010001 {
            FieldInfo::get_bit(
                iss,
                "IESB",
                Some("Implicit Error Synchronisation event"),
                13,
            )
            .describe_bit(describe_iesb)
        } else {
            FieldInfo::get_bit(iss, "RES0", Some("Reserved for this DFSC value"), 13)
                .check_res0()?
        };
        let aet = FieldInfo::get(iss, "AET", Some("Asynchronous Error Type"), 10, 13)
            .describe(describe_aet)?;
        let ea = FieldInfo::get_bit(iss, "EA", Some("External Abort type"), 9);
        let res0b = FieldInfo::get(iss, "RES0", Some("Reserved"), 6, 9).check_res0()?;
        vec![res0a, iesb, aet, ea, res0b, dfsc]
    };

    let mut fields = vec![ids];
    fields.extend(platform_fields);
    Ok(fields)
}

fn describe_ids(ids: bool) -> &'static str {
    if ids {
        "The rest of the ISS is encoded in an implementation-defined format"
    } else {
        "The rest of the ISS is encoded according to the platform"
    }
}

fn describe_iesb(iesb: bool) -> &'static str {
    if iesb {
        "The SError interrupt was synchronized by the implicit error synchronization event and taken immediately."
    } else {
        "The SError interrupt was not synchronized by the implicit error synchronization event or not taken immediately."
    }
}

fn describe_aet(aet: u64) -> Result<&'static str, DecodeError> {
    match aet {
        0b000 => Ok("Uncontainable (UC)"),
        0b001 => Ok("Unrecoverable state (UEU)"),
        0b010 => Ok("Restartable state (UEO)"),
        0b011 => Ok("Recoverable state (UER)"),
        0x110 => Ok("Corrected (CE)"),
        _ => Err(DecodeError::InvalidAet { aet }),
    }
}

fn describe_dfsc(dfsc: u64) -> Result<&'static str, DecodeError> {
    match dfsc {
        0b000000 => Ok("Uncategorized error"),
        0b010001 => Ok("Asynchronous SError interrupt"),
        _ => Err(DecodeError::InvalidFsc { fsc: dfsc }),
    }
}
