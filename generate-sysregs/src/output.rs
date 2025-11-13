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

pub fn write_all(mut writer: impl Write + Copy, registers: &[RegisterInfo]) -> io::Result<()> {
    writeln!(writer, "use bitflags::bitflags;")?;

    for register in registers {
        writeln!(writer)?;
        register.write_bitflags(writer)?;
    }
    writeln!(writer)?;
    for register in registers {
        register.write_accessor(writer)?;
    }

    Ok(())
}

impl RegisterInfo {
    fn write_bitflags(&self, mut writer: impl Write) -> io::Result<()> {
        writeln!(writer, "bitflags! {{")?;
        writeln!(writer, "    /// {} system register value.", self.name)?;
        writeln!(writer, "    #[derive(Clone, Copy, Debug, Eq, PartialEq)]")?;
        writeln!(writer, "    #[repr(transparent)]")?;
        writeln!(
            writer,
            "    pub struct {}: u{} {{",
            camel_case(&self.name),
            self.width
        )?;
        for bit in &self.bits {
            writeln!(writer, "        /// {} bit.", bit.name)?;
            writeln!(writer, "        const {} = 1 << {};", bit.name, bit.index)?;
        }
        writeln!(writer, "    }}")?;
        writeln!(writer, "}}")?;
        Ok(())
    }

    fn write_accessor(&self, mut writer: impl Write) -> io::Result<()> {
        match (self.read, self.write) {
            (None, None) => {}
            (None, Some(write_safety)) => {
                let safe_write = match write_safety {
                    Safety::Safe => ", safe",
                    Safety::Unsafe => "",
                };
                writeln!(
                    writer,
                    "write_sysreg!({}, u{}: {}{}, fake::SYSREGS);",
                    self.name.to_lowercase(),
                    self.width,
                    camel_case(&self.name),
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
                    "read_sysreg!({}, u{}: {}{}, fake::SYSREGS);",
                    self.name.to_lowercase(),
                    self.width,
                    camel_case(&self.name),
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
                    "read_write_sysreg!({}, u{}: {}{}{}, fake::SYSREGS);",
                    self.name.to_lowercase(),
                    self.width,
                    camel_case(&self.name),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camel_case() {
        assert_eq!(camel_case("SCR_EL3"), "ScrEl3");
        assert_eq!(camel_case("aBc_de_FGh_3a"), "AbcDeFgh3a");
    }
}
