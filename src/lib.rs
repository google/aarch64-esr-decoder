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

//! Library for decoding aarch64 Exception Syndrome Register and Main ID Register values.

mod esr;
mod midr;
mod smccc;

use bit_field::BitField;
pub use esr::decode;
pub use midr::decode_midr;
pub use smccc::decode_smccc;
use std::fmt::{self, Debug, Display, Formatter};
use std::num::ParseIntError;
use thiserror::Error;

/// Information about a particular field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FieldInfo {
    /// The short name of the field, e.g. "ISS".
    pub name: &'static str,
    /// The long name of the field, e.g. "Instruction Specific Syndrome".
    pub long_name: Option<&'static str>,
    /// The index of the lowest bit of the field.
    pub start: usize,
    /// The number of bits in the field.
    pub width: usize,
    /// The value of the field.
    pub value: u64,
    /// A description explaining the field value, if available.
    pub description: Option<String>,
    /// Any sub-fields.
    pub subfields: Vec<FieldInfo>,
}

impl FieldInfo {
    fn get(
        register: u64,
        name: &'static str,
        long_name: Option<&'static str>,
        start: usize,
        end: usize,
    ) -> Self {
        let value = register.get_bits(start..end);
        Self {
            name,
            long_name,
            start,
            width: end - start,
            value,
            description: None,
            subfields: vec![],
        }
    }

    fn get_bit(
        register: u64,
        name: &'static str,
        long_name: Option<&'static str>,
        bit: usize,
    ) -> Self {
        Self::get(register, name, long_name, bit, bit + 1)
    }

    fn with_description(self, description: String) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    fn as_bit(&self) -> bool {
        assert!(self.width == 1);
        self.value == 1
    }

    /// Assuming this field has a width of exactly 1, describe it with the given function.
    ///
    /// Panics if `self.width != 1`.
    fn describe_bit<F>(self, describer: F) -> Self
    where
        F: FnOnce(bool) -> &'static str,
    {
        let bit = self.as_bit();
        let description = describer(bit).to_string();
        self.with_description(description)
    }

    fn describe<F>(self, describer: F) -> Result<Self, DecodeError>
    where
        F: FnOnce(u64) -> Result<&'static str, DecodeError>,
    {
        let description = describer(self.value)?.to_string();
        Ok(self.with_description(description))
    }

    fn check_res0(self) -> Result<Self, DecodeError> {
        if self.value != 0 {
            Err(DecodeError::InvalidRes0 { res0: self.value })
        } else {
            Ok(self)
        }
    }

    /// Returns the value as a hexadecimal string, or "true" or "false" if it is a single bit.
    pub fn value_string(&self) -> String {
        if self.width == 1 {
            if self.value == 1 { "true" } else { "false" }.to_string()
        } else {
            format!("{:#01$x}", self.value, (self.width + 3) / 4 + 2,)
        }
    }

    /// Returns the value as a binary strings.
    pub fn value_binary_string(&self) -> String {
        format!("{:#01$b}", self.value, self.width + 2)
    }
}

impl Display for FieldInfo {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.width == 1 {
            write!(
                f,
                "{}: {}",
                self.name,
                if self.value == 1 { "true" } else { "false" }
            )
        } else {
            write!(
                f,
                "{}: {} {}",
                self.name,
                self.value_string(),
                self.value_binary_string(),
            )
        }
    }
}

/// An error decoding a register value.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// A RES0 field was not 0.
    #[error("Invalid ESR, res0 is {res0:#x}")]
    InvalidRes0 { res0: u64 },
    /// The EC field had an invalid value.
    #[error("Invalid EC {ec:#x}")]
    InvalidEc { ec: u64 },
    /// The DFSC or IFSC field had an invalid value.
    #[error("Invalid DFSC or IFSC {fsc:#x}")]
    InvalidFsc { fsc: u64 },
    /// The SET field had an invalid value.
    #[error("Invalid SET {set:#x}")]
    InvalidSet { set: u64 },
    /// The AET field had an invalid value.
    #[error("Invalid AET {aet:#x}")]
    InvalidAet { aet: u64 },
    /// The AM field had an invalid value.
    #[error("Invalid AM {am:#x}")]
    InvalidAm { am: u64 },
    /// The ISS field has an invalid value for a trapped LD64B or ST64B* exception.
    #[error("Invalid ISS {iss:#x} for trapped LD64B or ST64B*")]
    InvalidLd64bIss { iss: u64 },
}

/// Parses a decimal or hexadecimal number from a string.
///
/// If the string starts with `"0x"` then it will be parsed as hexadecimal, otherwise it will be
/// assumed to be decimal.
pub fn parse_number(s: &str) -> Result<u64, ParseIntError> {
    if let Some(hex) = s.strip_prefix("0x") {
        u64::from_str_radix(hex, 16)
    } else {
        s.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_decimal() {
        assert_eq!(parse_number("12345"), Ok(12345));
    }

    #[test]
    fn parse_hex() {
        assert_eq!(parse_number("0x123abc"), Ok(0x123abc));
    }

    #[test]
    fn parse_invalid() {
        assert!(parse_number("123abc").is_err());
    }
}
