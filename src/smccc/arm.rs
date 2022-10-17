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

use super::{reserved_fids, DecodeError, FieldInfo};

pub fn decode_arm_service(smccc: u64, conv: u64) -> Result<FieldInfo, DecodeError> {
    if conv == 0 {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_arm32_service)
    } else {
        FieldInfo::get(smccc, "Function Number", None, 0, 16).describe(describe_arm64_service)
    }
}

fn describe_arm32_service(service: u64) -> Result<&'static str, DecodeError> {
    Ok(match service {
        0x0000 => "SMCCC_VERSION",
        0x0001 => "SMCCC_ARCH_FEATURES",
        0x0002 => "SMCCC_ARCH_SOC_ID",
        0x3FFF => "SMCCC_ARCH_WORKAROUND_3",
        0x7FFF => "SMCCC_ARCH_WORKAROUND_2",
        0x8000 => "SMCCC_ARCH_WORKAROUND_1",
        0xFF00 => "Call Count Query, deprecated from SMCCCv1.2",
        0xFF01 => "Call UUID Query, deprecated from SMCCCv1.2",
        0xFF03 => "Revision Query, deprecated from SMCCCv1.2",
        _ => reserved_fids(service),
    })
}
fn describe_arm64_service(service: u64) -> Result<&'static str, DecodeError> {
    Ok(reserved_fids(service))
}
