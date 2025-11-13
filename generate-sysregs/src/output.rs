// Copyright 2025 Google LLC
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

//! Logic for writing out a Rust source file with system register types and accessors.

use crate::{RegisterInfo, Safety};
use std::io::{self, Write};

pub fn write_lib(mut writer: impl Write + Copy, registers: &[RegisterInfo]) -> io::Result<()> {
    writer.write_all(
        "\
//! Access to Arm CPU system registers.

#![cfg_attr(not(any(test, feature = \"fakes\")), no_std)]

#[cfg(not(any(test, feature = \"fakes\")))]
mod aarch64;
#[cfg(any(test, feature = \"fakes\"))]
pub mod fake;
mod macros;

#[doc(hidden)]
pub use paste as _paste;

use bitflags::bitflags;
"
        .as_bytes(),
    )?;

    for register in registers {
        if register.use_struct() {
            writeln!(writer)?;
            register.write_bitflags(writer)?;
            register.write_impl(writer)?;
        }
    }
    writeln!(writer)?;
    for register in registers {
        register.write_accessor(writer)?;
    }

    Ok(())
}

pub fn write_fake(mut writer: impl Write + Copy, registers: &[RegisterInfo]) -> io::Result<()> {
    writeln!(writer, "/// A set of fake system registers.")?;
    writeln!(writer, "#[derive(Clone, Debug, Eq, PartialEq)]")?;
    writeln!(writer, "pub struct SystemRegisters {{")?;
    for register in registers {
        writeln!(
            writer,
            "    /// Fake value for the {} system register.",
            register.name
        )?;
        let register_type = if register.use_struct() {
            register.struct_name()
        } else {
            format!("u{}", register.width)
        };
        writeln!(
            writer,
            "    pub {}: {},",
            register.name.to_lowercase(),
            register_type
        )?;
    }
    writeln!(writer, "}}")?;
    writeln!(writer)?;
    writeln!(writer, "impl SystemRegisters {{")?;
    writeln!(writer, "    const fn new() -> Self {{")?;
    writeln!(writer, "        Self {{")?;
    for register in registers {
        if register.use_struct() {
            writeln!(
                writer,
                "            {}: {}::empty(),",
                register.name.to_lowercase(),
                register.struct_name(),
            )?;
        } else {
            writeln!(writer, "            {}: 0,", register.name.to_lowercase(),)?;
        }
    }
    writeln!(writer, "        }}")?;
    writeln!(writer, "    }}")?;
    writeln!(writer, "}}")?;

    Ok(())
}

impl RegisterInfo {
    /// Whether to use a wrapper bitflags struct type for the register, rather than just a raw
    /// primitive type.
    fn use_struct(&self) -> bool {
        !self.fields.is_empty()
    }

    /// The name to use for the struct type for the register.
    fn struct_name(&self) -> String {
        camel_case(&self.name)
    }

    fn write_bitflags(&self, mut writer: impl Write) -> io::Result<()> {
        writeln!(writer, "bitflags! {{")?;
        writeln!(writer, "    /// {} system register value.", self.name)?;
        writeln!(writer, "    #[derive(Clone, Copy, Debug, Eq, PartialEq)]")?;
        writeln!(writer, "    #[repr(transparent)]")?;
        writeln!(
            writer,
            "    pub struct {}: u{} {{",
            self.struct_name(),
            self.width
        )?;
        for field in &self.fields {
            if field.width == 1 {
                writeln!(writer, "        /// {} bit.", field.name)?;
                writeln!(
                    writer,
                    "        const {} = 1 << {};",
                    field.name.to_uppercase(),
                    field.index,
                )?;
            }
        }
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn write_impl(&self, mut writer: impl Write) -> io::Result<()> {
        if self.fields.iter().any(|field| field.width > 1) {
            writeln!(writer)?;
            writeln!(writer, "impl {} {{", self.struct_name())?;
            let mut first = true;
            for field in &self.fields {
                if field.width > 1 {
                    if !first {
                        writeln!(writer)?;
                        first = false;
                    }

                    let field_type = type_for_width(field.width);

                    writeln!(
                        writer,
                        "    /// Returns the value of the {} field.",
                        field.name
                    )?;
                    writeln!(
                        writer,
                        "    pub fn {}(self) -> {} {{",
                        field.name.to_lowercase(),
                        field_type
                    )?;
                    writeln!(
                        writer,
                        "        (self.bits() >> {}) as {} & {:#b}",
                        field.index,
                        field_type,
                        u64::MAX >> (64 - field.width),
                    )?;
                    writeln!(writer, "    }}")?;
                }
            }
            writeln!(writer, "}}")?;
        }
        Ok(())
    }

    fn write_accessor(&self, mut writer: impl Write) -> io::Result<()> {
        let register_type = if self.use_struct() {
            format!("u{}: {}", self.width, self.struct_name())
        } else {
            format!("u{}", self.width)
        };
        match (self.read, self.write) {
            (None, None) => {}
            (None, Some(write_safety)) => {
                let safe_write = match write_safety {
                    Safety::Safe => ", safe",
                    Safety::Unsafe => "",
                };
                writeln!(
                    writer,
                    "write_sysreg!({}, {}{}, fake::SYSREGS);",
                    self.name.to_lowercase(),
                    register_type,
                    safe_write,
                )?;
            }
            (Some(read_safety), None) => {
                let safe_read = match read_safety {
                    Safety::Safe => ", safe",
                    Safety::Unsafe => "",
                };
                writeln!(
                    writer,
                    "read_sysreg!({}, {}{}, fake::SYSREGS);",
                    self.name.to_lowercase(),
                    register_type,
                    safe_read,
                )?;
            }
            (Some(read_safety), Some(write_safety)) => {
                let safe_read = match read_safety {
                    Safety::Safe => ", safe_read",
                    Safety::Unsafe => "",
                };
                let safe_write = match write_safety {
                    Safety::Safe => ", safe_write",
                    Safety::Unsafe => "",
                };
                writeln!(
                    writer,
                    "read_write_sysreg!({}, {}{}{}, fake::SYSREGS);",
                    self.name.to_lowercase(),
                    register_type,
                    safe_read,
                    safe_write,
                )?;
            }
        }
        Ok(())
    }
}

fn camel_case(name: &str) -> String {
    name.split('_')
        .flat_map(|part| [part[0..1].to_uppercase(), part[1..].to_lowercase()])
        .collect()
}

/// Returns the smallest unsigned type that can hold at least the given number of bits
fn type_for_width(width: u32) -> &'static str {
    if width > 32 {
        "u64"
    } else if width > 16 {
        "u32"
    } else if width > 8 {
        "u16"
    } else {
        "u8"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("SCR_EL3"), "ScrEl3");
        assert_eq!(camel_case("aBc_de_FGh_3a"), "AbcDeFgh3a");
    }
}
