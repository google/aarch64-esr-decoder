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

use arm_sysregs_xml::{EncName, Encoding, ExecutionState, RegisterPage};
use quick_xml::de;
use std::{
    collections::BTreeMap,
    fs::{File, read_dir},
    io::BufReader,
};

fn main() {
    let mut encodings = BTreeMap::new();
    for entry in read_dir("SysReg_xml_A_profile-2025-06/SysReg_xml_A_profile-2025-06").unwrap() {
        let entry = entry.unwrap();
        let filename = entry.file_name().into_string().unwrap();
        if filename.ends_with(".xml")
            && !filename.ends_with("index.xml")
            && ![
                "amu.xml",
                "architecture_info.xml",
                "instructions.xml",
                "notice.xml",
                "pmu.xml",
            ]
            .contains(&filename.as_str())
        {
            let register_page = de::from_reader::<_, RegisterPage>(BufReader::new(
                File::open(entry.path()).unwrap(),
            ))
            .unwrap();
            let register = &register_page.registers.register;
            if register.execution_state != Some(ExecutionState::AArch64) {
                continue;
            }
            for mechanism in &register.access_mechanisms.access_mechanism {
                if let Some(encoding) = &mechanism.encoding
                    && encoding.access_instruction.starts_with("MRS <Xt>, ")
                {
                    let reg_name = &encoding.access_instruction[10..];
                    if let (Some(op0), Some(op1), Some(crn), Some(crm), Some(op2)) = (
                        enc_value(encoding, EncName::Op0),
                        enc_value(encoding, EncName::Op1),
                        enc_value(encoding, EncName::CRn),
                        enc_value(encoding, EncName::CRm),
                        enc_value(encoding, EncName::Op2),
                    ) {
                        encodings.insert(
                            reg_name.to_owned(),
                            format!("({op0}, {crn}, {op1}, {crm}, {op2})"),
                        );
                    } else {
                        println!("// {reg_name}");
                    }
                }
            }
        }
    }
    for (name, encoding) in &encodings {
        println!("{encoding} => \"{name}\",");
    }
}

fn enc_value(encoding: &Encoding, name: EncName) -> Option<u8> {
    encoding.enc.iter().find(|enc| enc.n == name)?.parse_value()
}
