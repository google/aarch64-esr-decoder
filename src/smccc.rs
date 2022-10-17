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

// Decoder for SMC Calling Convention ARM DEN 0028E v1.4

mod arm;
mod common;
mod ffa;
mod hyp;
mod secure;
mod tapp;

use arm::decode_arm_service;
use common::decode_common_service;
use common::describe_general32_queries;
use common::reserved_fids;
use common::smccc_general32_queries;
use hyp::decode_hyp_service;
use secure::decode_secure_service;
use tapp::decode_tapp_service;

use super::{DecodeError, FieldInfo};

/// Decodes the function ID of an SMCCC (ARM DEN 0028E v1.4) call, or returns an error if it is not valid.
pub fn decode_smccc(smccc: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let call_type = FieldInfo::get(smccc, "Call Type", None, 31, 32).describe(describe_call)?;

    let result = if call_type.value == 1 {
        parse_fastcall(smccc)?
    } else {
        parse_yieldcall(smccc)?
    };

    Ok([vec![call_type], result].concat())
}

pub fn parse_fastcall(smccc: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let call_convention =
        FieldInfo::get(smccc, "Call Convention", None, 30, 31).describe(describe_convention)?;
    let service_call =
        FieldInfo::get(smccc, "Service Call", None, 24, 30).describe(describe_service)?;

    let mbz = FieldInfo::get(
        smccc,
        "MBZ",
        Some("Some legacy Armv7 set this to 1"),
        17,
        24,
    );
    let sve = FieldInfo::get(
        smccc,
        "SVE live state",
        Some("No live state[1] From SMCCCv1.3, before SMCCCv1.3 MBZ"),
        16,
        17,
    );

    let function_number = match service_call.value {
        0x00 => decode_arm_service(smccc, call_convention.value)?,
        0x04 => decode_secure_service(smccc, call_convention.value)?,
        0x05 => decode_hyp_service(smccc, call_convention.value)?,
        0x30..=0x31 => decode_tapp_service(smccc, call_convention.value)?,
        _ => decode_common_service(smccc, call_convention.value)?,
    };

    Ok(vec![
        call_convention,
        service_call,
        mbz,
        sve,
        function_number,
    ])
}
pub fn parse_yieldcall(smccc: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let yield_type =
        FieldInfo::get(smccc, "Service Type", None, 0, 31).describe(describe_yield_service)?;
    Ok(vec![yield_type])
}

fn describe_yield_service(service: u64) -> Result<&'static str, DecodeError> {
    Ok(match service {
        0x00000000..=0x0100FFFF => {
            "Reserved for existing APIs (in use by the existing Armv7 devices)"
        }
        0x02000000..=0x1FFFFFFF => "Trusted OS Yielding Calls",
        0x20000000..=0x7FFFFFFF => "Reserved for future expansion of Trusted OS Yielding Calls",
        _ => "Unknown",
    })
}

fn describe_call(call: u64) -> Result<&'static str, DecodeError> {
    Ok(match call {
        0x00 => "Yielding Call",
        0x01 => "Fast Call",
        _ => "Unknown",
    })
}
fn describe_convention(conv: u64) -> Result<&'static str, DecodeError> {
    Ok(match conv {
        0x00 => "SMC32/HVC32",
        0x01 => "SMC64/HVC64",
        _ => "Unknown",
    })
}
fn describe_service(service: u64) -> Result<&'static str, DecodeError> {
    Ok(match service {
        0x00 => "Arm Architecture Call",
        0x01 => "CPU Service Call",
        0x02 => "SiP Service Call",
        0x03 => "OEM Service Call",
        0x04 => "Standard Secure Service Call",
        0x05 => "Standard Hypervisor Service Call",
        0x06 => "Vendor Specific Hypervisor Service Call",
        0x07..=0x2F => "Reserved for future use",
        0x30..=0x31 => "Trusted Application Call",
        0x32..=0x3F => "Trusted OS Call",
        _ => "Unknown",
    })
}
