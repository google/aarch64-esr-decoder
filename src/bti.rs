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

/// Decodes the ISS value for a Branch Target Exception.
pub fn decode_iss_bti(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0 = FieldInfo::get(iss, "RES0", Some("Reserved"), 2, 25).check_res0()?;
    let btype = FieldInfo::get(iss, "BTYPE", Some("PSTATE.BTYPE value"), 0, 2);

    Ok(vec![res0, btype])
}
