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

use super::{describe_general32_queries, DecodeError, FieldInfo};

pub fn decode_hyp_service(smccc: u64, conv: u64) -> Result<FieldInfo, DecodeError> {
    if conv == 0 {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_general32_queries)
    } else {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_hyp64_service)
    }
}

fn describe_hyp64_service(service: u64) -> Result<&'static str, DecodeError> {
    Ok(match service {
        0x20..=0x3F => "PV Time 64-bit calls",
        _ => "",
    })
}
