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

use super::{
    ffa::{ffa_32_function_id, ffa_64_function_id},
    smccc_general32_queries, DecodeError, FieldInfo,
};

pub fn decode_secure_service(smccc: u64, conv: u64) -> Result<FieldInfo, DecodeError> {
    let info = if conv == 0 {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_secure32_service)?
    } else {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_secure64_service)?
    };
    Ok(info)
}

fn secure_service(service: u64) -> &'static str {
    match service {
        0x000..=0x01F => "PSCI Call (Power Secure Control Interface)",
        0x020..=0x03F => "SDEI Call (Software Delegated Exception Interface)",
        0x040..=0x04F => "MM Call (Management Mode)",
        0x050..=0x05F => "TRNG Call",
        0x060..=0x0EF => "Unknown FF-A Call",
        0x0F0..=0x10F => "Errata Call",
        0x150..=0x1CF => "CCA Call",
        _ => "",
    }
}

fn describe_secure32_service(service: u64) -> Result<&'static str, DecodeError> {
    if let Some(ffa_call) = ffa_32_function_id(service) {
        return Ok(ffa_call);
    }

    Ok(match service {
        0x000..=0x1CF => secure_service(service),
        _ => smccc_general32_queries(service),
    })
}
fn describe_secure64_service(service: u64) -> Result<&'static str, DecodeError> {
    if let Some(ffa_call) = ffa_64_function_id(service) {
        return Ok(ffa_call);
    }
    Ok(secure_service(service))
}
