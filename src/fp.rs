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

/// Decodes the ISS value for a floating-point exception.
pub fn decode_iss_fp(iss: u64) -> Result<Vec<FieldInfo>, DecodeError> {
    let res0a = FieldInfo::get_bit(iss, "RES0", Some("Reserved"), 24).check_res0()?;
    let tfv =
        FieldInfo::get_bit(iss, "TFV", Some("Trapped Fault Valid"), 23).describe_bit(describe_tfv);
    let res0b = FieldInfo::get(iss, "RES0", Some("Reserved"), 11, 23).check_res0()?;
    let vecitr = FieldInfo::get(iss, "VECITR", Some("RES1 or UNKNOWN"), 8, 11);
    let idf = FieldInfo::get_bit(iss, "IDF", Some("Input Denormal"), 7).describe_bit(describe_idf);
    let res0c = FieldInfo::get(iss, "RES0", Some("Reserved"), 5, 7).check_res0()?;
    let ixf = FieldInfo::get_bit(iss, "IXF", Some("Inexact"), 4).describe_bit(describe_ixf);
    let uff = FieldInfo::get_bit(iss, "UFF", Some("Underflow"), 3).describe_bit(describe_uff);
    let off = FieldInfo::get_bit(iss, "OFF", Some("Overflow"), 2).describe_bit(describe_off);
    let dzf = FieldInfo::get_bit(iss, "DZF", Some("Divide by Zero"), 1).describe_bit(describe_dzf);
    let iof =
        FieldInfo::get_bit(iss, "IOF", Some("Invalid Operation"), 0).describe_bit(describe_iof);

    Ok(vec![
        res0a, tfv, res0b, vecitr, idf, res0c, ixf, uff, off, dzf, iof,
    ])
}

fn describe_tfv(tfv: bool) -> &'static str {
    if tfv {
        "One or more floating-point exceptions occurred; IDF, IXF, UFF, OFF, DZF and IOF hold information about what."
    } else {
        "IDF, IXF, UFF, OFF, DZF and IOF do not hold valid information."
    }
}

fn describe_idf(idf: bool) -> &'static str {
    if idf {
        "Input denormal floating-point exception occurred."
    } else {
        "Input denormal floating-point exception did not occur."
    }
}

fn describe_ixf(ixf: bool) -> &'static str {
    if ixf {
        "Inexact floating-point exception occurred."
    } else {
        "Inexact floating-point exception did not occur."
    }
}

fn describe_uff(uff: bool) -> &'static str {
    if uff {
        "Underflow floating-point exception occurred."
    } else {
        "Underflow floating-point exception did not occur."
    }
}

fn describe_off(off: bool) -> &'static str {
    if off {
        "Overflow floating-point exception occurred."
    } else {
        "Overflow floating-point exception did not occur."
    }
}

fn describe_dzf(dzf: bool) -> &'static str {
    if dzf {
        "Divide by Zero floating-point exception occurred."
    } else {
        "Divide by Zero floating-point exception did not occur."
    }
}

fn describe_iof(iof: bool) -> &'static str {
    if iof {
        "Invalid Operation floating-point exception occurred."
    } else {
        "Invalid Operation floating-point exception did not occur."
    }
}
