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

use super::{DecodeError, FieldInfo};

/// Decodes the given Main ID Register value, or returns an error if it is not valid.
pub fn decode_midr(midr: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(midr, "RES0", Some("Reserved"), 32, 64).check_res0()?;
    let implementer =
        FieldInfo::get(midr, "Implementer", None, 24, 32).describe(describe_implementer)?;
    let variant = FieldInfo::get(midr, "Variant", None, 20, 24);
    let architecture =
        FieldInfo::get(midr, "Architecture", None, 16, 20).describe(describe_architecture)?;
    let part_num = FieldInfo::get(midr, "PartNum", Some("Part number"), 4, 16);
    let revision = FieldInfo::get(midr, "Revision", None, 0, 4);

    Ok(vec![
        res0,
        implementer,
        variant,
        architecture,
        part_num,
        revision,
    ])
}

fn describe_implementer(implementer: u64) -> Result<&'static str, DecodeError> {
    Ok(match implementer {
        0x00 => "Reserved for software use",
        0xC0 => "Ampere Computing",
        0x41 => "Arm Limited",
        0x42 => "Broadcom Corporation",
        0x43 => "Cavium Inc.",
        0x44 => "Digital Equipment Corporation",
        0x46 => "Fujitsu Ltd.",
        0x49 => "Infineon Technologies AG",
        0x4D => "Motorola or Freescale Semiconductor Inc.",
        0x4E => "NVIDIA Corporation",
        0x50 => "Applied Micro Circuits Corporation",
        0x51 => "Qualcomm Inc.",
        0x56 => "Marvell International Ltd.",
        0x69 => "Intel Corporation",
        _ => "Unknown",
    })
}

fn describe_architecture(architecture: u64) -> Result<&'static str, DecodeError> {
    Ok(match architecture {
        0b0001 => "Armv4",
        0b0010 => "Armv4T",
        0b0011 => "Armv5",
        0b0100 => "Armv5T",
        0b0101 => "Armv5TE",
        0b0110 => "Armv5TEJ",
        0b0111 => "Armv6",
        0b1111 => "Architectural features are individually identified",
        _ => "Reserved",
    })
}
