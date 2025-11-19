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

use crate::{RegisterField, RegisterInfo, Safety, ones};
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
mod manual;

use bitflags::bitflags;
pub use manual::*;
#[doc(hidden)]
pub use paste as _paste;
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
    writeln!(writer, "#[derive(Clone, Debug, Default, Eq, PartialEq)]")?;
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
            register.variable_name(),
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
                register.variable_name(),
                register.struct_name(),
            )?;
        } else {
            writeln!(writer, "            {}: 0,", register.variable_name())?;
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
        if self.res1 != 0 {
            writeln!(
                writer,
                "        /// RES1 bits in the {} register.",
                self.name
            )?;
            writeln!(writer, "        const RES1 = {:#b};", self.res1)?;
        }
        for field in &self.fields {
            if field.width == 1 {
                if let Some(array_info) = &field.array_info {
                    let placeholder = array_info.placeholder();
                    for i in array_info.indices.clone() {
                        writeln!(writer, "        /// {} bit {}.", field.name, i)?;
                        if let Some(description) = &field.description {
                            writeln!(writer, "        ///")?;
                            writeln!(writer, "        /// {}", description)?;
                        }
                        writeln!(
                            writer,
                            "        const {} = 1 << {};",
                            field
                                .constant_name()
                                .replace(&placeholder, &format!("{}", i)),
                            field.index,
                        )?;
                    }
                } else {
                    if let Some(description) = &field.description {
                        writeln!(writer, "        /// {}", description)?;
                    } else {
                        writeln!(writer, "        /// {} bit.", field.name)?;
                    }
                    writeln!(
                        writer,
                        "        const {} = 1 << {};",
                        field.constant_name(),
                        field.index,
                    )?;
                }
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
                    if first {
                        first = false;
                    } else {
                        writeln!(writer)?;
                    }

                    let field_type = type_for_width(field.width);

                    if let Some(array_info) = &field.array_info {
                        writeln!(
                            writer,
                            "    /// Returns the value of the given {} field.",
                            field.name,
                        )?;
                        if let Some(description) = &field.description {
                            writeln!(writer, "    ///")?;
                            writeln!(writer, "    /// {description}")?;
                        }
                        writeln!(
                            writer,
                            "    pub const fn {}(self, {}: u32) -> {} {{",
                            field.function_name().replace(&array_info.placeholder(), ""),
                            array_info.index_variable,
                            field_type,
                        )?;
                        if array_info.indices.start > 0 {
                            writeln!(
                                writer,
                                "        assert!({} >= {} && {} < {});",
                                array_info.index_variable,
                                array_info.indices.start,
                                array_info.index_variable,
                                array_info.indices.end,
                            )?;
                        } else {
                            writeln!(
                                writer,
                                "        assert!({} < {});",
                                array_info.index_variable, array_info.indices.end,
                            )?;
                        }
                        writeln!(
                            writer,
                            "        (self.bits() >> ({} + ({} - {}) * {})) as {} & {:#b}",
                            field.index,
                            array_info.index_variable,
                            array_info.indices.start,
                            field.width,
                            field_type,
                            ones(field.width),
                        )?;
                        writeln!(writer, "    }}")?;
                    } else {
                        writeln!(
                            writer,
                            "    /// Returns the value of the {} field.",
                            field.name
                        )?;
                        if let Some(description) = &field.description {
                            writeln!(writer, "    ///")?;
                            writeln!(writer, "    /// {description}")?;
                        }
                        writeln!(
                            writer,
                            "    pub const fn {}(self) -> {} {{",
                            field.function_name(),
                            field_type
                        )?;
                        writeln!(
                            writer,
                            "        (self.bits() >> {}) as {} & {:#b}",
                            field.index,
                            field_type,
                            ones(field.width),
                        )?;
                        writeln!(writer, "    }}")?;
                    }
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
                if let Some(safety_doc) = &self.write_safety_doc {
                    writeln!(
                        writer,
                        "\
write_sysreg! {{
    /// # Safety
    ///
    /// {}
    {}, {}{}, fake::SYSREGS
}}",
                        safety_doc,
                        self.variable_name(),
                        register_type,
                        safe_write,
                    )?;
                } else {
                    writeln!(
                        writer,
                        "write_sysreg!({}, {}{}, fake::SYSREGS);",
                        self.variable_name(),
                        register_type,
                        safe_write,
                    )?;
                }
            }
            (Some(read_safety), None) => {
                let safe_read = match read_safety {
                    Safety::Safe => ", safe",
                    Safety::Unsafe => "",
                };
                writeln!(
                    writer,
                    "read_sysreg!({}, {}{}, fake::SYSREGS);",
                    self.variable_name(),
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
                if let Some(safety_doc) = &self.write_safety_doc {
                    writeln!(
                        writer,
                        "\
read_write_sysreg! {{
    /// # Safety
    ///
    /// {}
    {}, {}{}{}, fake::SYSREGS
}}",
                        safety_doc,
                        self.variable_name(),
                        register_type,
                        safe_read,
                        safe_write,
                    )?;
                } else {
                    writeln!(
                        writer,
                        "read_write_sysreg!({}, {}{}{}, fake::SYSREGS);",
                        self.variable_name(),
                        register_type,
                        safe_read,
                        safe_write,
                    )?;
                }
            }
        }
        Ok(())
    }

    /// Returns the name of the field formatted to be a valid Rust variable name.
    fn variable_name(&self) -> String {
        lowercase_name(&self.name)
    }
}

impl RegisterField {
    /// Returns the name of the field formatted to be a valid Rust constant name.
    fn constant_name(&self) -> String {
        uppercase_name(&self.name)
    }

    /// Returns the name of the field formatted to be a valid Rust function name.
    fn function_name(&self) -> String {
        lowercase_name(&self.name)
    }
}

fn camel_case(name: &str) -> String {
    name.split('_')
        .flat_map(|part| [part[0..1].to_uppercase(), part[1..].to_lowercase()])
        .collect()
}

fn lowercase_name(name: &str) -> String {
    name.replace(':', "_")
        .replace('[', "_")
        .replace(']', "")
        .to_lowercase()
}

fn uppercase_name(name: &str) -> String {
    name.replace(':', "_")
        .replace('[', "_")
        .replace(']', "")
        .to_uppercase()
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
