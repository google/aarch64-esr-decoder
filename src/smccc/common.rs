// Copyright 2022 Google LLC
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

use super::{DecodeError, FieldInfo};

pub fn decode_common_service(smccc: u64, conv: u64) -> Result<FieldInfo, DecodeError> {
    if conv == 0 {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_general32_queries)
    } else {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_general64_queries)
    }
}

pub fn reserved_fids(service: u64) -> &'static str {
    match service {
        0xFF00..=0xFFFF => "Reserved for future expansion",
        _ => "",
    }
}

pub fn smccc_general32_queries(service: u64) -> &'static str {
    match service {
        0xFF00 => "Call Count Query, deprecated from SMCCCv1.2",
        0xFF01 => "Call UUID Query",
        0xFF03 => "Revision Query",
        _ => reserved_fids(service),
    }
}
pub fn describe_general32_queries(service: u64) -> Result<&'static str, DecodeError> {
    Ok(smccc_general32_queries(service))
}
pub fn describe_general64_queries(service: u64) -> Result<&'static str, DecodeError> {
    Ok(reserved_fids(service))
}
