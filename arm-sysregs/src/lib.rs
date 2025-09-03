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

use serde::{Deserialize, Deserializer, de::Unexpected};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegisterPage {
    pub registers: Registers,
    pub timestamp: String,
    pub commit_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Registers {
    pub register: Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Register {
    #[serde(rename = "@execution_state")]
    pub execution_state: Option<ExecutionState>,
    #[serde(rename = "@is_register", deserialize_with = "titlecase_bool")]
    pub is_register: bool,
    #[serde(rename = "@is_internal", deserialize_with = "titlecase_bool")]
    pub is_internal: bool,
    #[serde(rename = "@is_stub_entry", deserialize_with = "titlecase_bool")]
    pub is_stub_entry: bool,
    pub reg_short_name: String,
    pub reg_long_name: String,
    pub reg_condition: Option<RegCondition>,
    pub power_domain_text: Option<Text>,
    pub reg_reset_value: RegResetValue,
    pub reg_mappings: RegMappings,
    pub reg_purpose: RegPurpose,
    pub reg_groups: RegGroups,
    pub reg_configuration: Option<RegConfiguration>,
    pub reg_attributes: RegAttributes,
    pub reg_fieldsets: RegFieldsets,
    pub access_mechanisms: AccessMechanisms,
    pub arch_variants: ArchVariants,
    #[serde(default)]
    pub reg_address: Vec<RegAddress>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum ExecutionState {
    AArch32,
    AArch64,
    External,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegCondition {
    #[serde(rename = "@otherwise")]
    pub otherwise: Option<String>,
    #[serde(rename = "$value", default)]
    pub condition: Vec<TextEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegResetValue {
    #[serde(default)]
    pub reg_reset_limited_to_el: Vec<String>,
    pub reg_reset_special_text: Option<RegResetSpecialText>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegResetSpecialText {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegMappings {
    #[serde(default)]
    pub reg_mapping: Vec<RegMapping>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegMapping {
    pub mapped_name: MappedName,
    pub mapped_type: String,
    pub mapped_execution_state: ExecutionState,
    pub mapped_from_startbit: Option<u8>,
    pub mapped_from_endbit: Option<u8>,
    pub mapped_to_startbit: Option<u8>,
    pub mapped_to_endbit: Option<u8>,
    pub mapped_from_rangeset: Option<MappedFromRangeset>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct MappedName {
    #[serde(rename = "@filename")]
    pub filename: String,
    #[serde(rename = "$text")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct MappedFromRangeset {
    #[serde(rename = "@output")]
    pub output: String,
    pub range: Vec<Range>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Range {
    pub msb: u8,
    pub lsb: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegPurpose {
    pub purpose_text: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegGroups {
    pub reg_group: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegConfiguration {
    #[serde(default)]
    pub configuration_text: Vec<ConfigurationText>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ConfigurationText {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegAttributes {
    pub attributes_text: Vec<Text>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegFieldsets {
    #[serde(default)]
    pub fields: Vec<Fields>,
    #[serde(default)]
    pub reg_fieldset: Vec<RegFieldset>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Fields {
    #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@length")]
    pub length: Option<u8>,
    pub fields_condition: Option<String>,
    pub text_before_fields: Text,
    pub field: Vec<Field>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Field {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@has_partial_fieldset", deserialize_with = "titlecase_bool")]
    pub has_partial_fieldset: bool,
    #[serde(
        rename = "@is_linked_to_partial_fieldset",
        deserialize_with = "titlecase_bool"
    )]
    pub is_linked_to_partial_fieldset: bool,
    #[serde(
        rename = "@is_access_restriction_possible",
        deserialize_with = "titlecase_bool"
    )]
    pub is_access_restriction_possible: bool,
    #[serde(rename = "@is_variable_length", deserialize_with = "titlecase_bool")]
    pub is_variable_length: bool,
    #[serde(rename = "@is_constant_value", deserialize_with = "titlecase_bool")]
    pub is_constant_value: bool,
    #[serde(rename = "@is_partial_field", deserialize_with = "titlecase_bool")]
    pub is_partial_field: bool,
    #[serde(
        rename = "@is_conditional_field_name",
        deserialize_with = "titlecase_bool"
    )]
    pub is_conditional_field_name: bool,
    #[serde(rename = "@rwtype")]
    pub rwtype: Option<String>,
    pub field_name: Option<String>,
    pub field_msb: u8,
    pub field_lsb: u8,
    pub rel_range: String,
    pub field_description: Vec<FieldDescription>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FieldDescription {
    #[serde(rename = "@order")]
    pub order: Order,
    #[serde(rename = "$value", default)]
    pub description: Vec<TextEntry>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq)]
pub struct Text {
    #[serde(rename = "$value", default)]
    pub text: Vec<TextEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum TextEntry {
    #[serde(rename = "$text")]
    String(String),
    ArmDefinedWord(String),
    List(List),
    Note(Text),
    Para(Para),
    Table(Table),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Para {
    // TODO
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct List {
    pub listitem: Vec<ListItem>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ListItem {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Table {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    After,
    Before,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegFieldset {
    #[serde(rename = "@length")]
    pub length: u8,
    pub fields_condition: Option<String>,
    pub fieldat: Vec<FieldAt>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct FieldAt {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@label")]
    pub label: Option<String>,
    #[serde(rename = "@msb")]
    pub msb: u8,
    #[serde(rename = "@lsb")]
    pub lsb: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessMechanisms {
    #[serde(default)]
    pub access_mechanism: Vec<AccessMechanism>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessMechanism {
    #[serde(rename = "@accessor")]
    pub accessor: Option<String>,
    #[serde(rename = "@type")]
    pub type_: AccessMechanismType,
    #[serde(rename = "@table_id")]
    pub table_id: Option<String>,
    pub encoding: Option<Encoding>,
    pub access_permission: Option<AccessPermission>,
    pub access_header: Option<AccessHeader>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum AccessMechanismType {
    BlockAccessAbstract,
    SystemAccessor,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Encoding {
    pub access_instruction: String,
    pub enc: Vec<Enc>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Enc {
    #[serde(rename = "@n")]
    pub n: EncName,
    #[serde(rename = "@v")]
    pub v: String,
}

impl Enc {
    pub fn parse_value(&self) -> Option<u8> {
        let (prefix, rest) = self.v.split_at_checked(2)?;
        if prefix == "0b" {
            u8::from_str_radix(rest, 2).ok()
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncName {
    Coproc,
    #[serde(rename = "CRd")]
    CRd,
    #[serde(rename = "CRm")]
    CRm,
    #[serde(rename = "CRn")]
    CRn,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "M1")]
    M1,
    Op0,
    Op1,
    Op2,
    Opc1,
    Opc2,
    #[serde(rename = "R")]
    R,
    Reg,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessPermission {
    pub ps: Ps,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessHeader {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Ps {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@sections")]
    pub sections: usize,
    #[serde(rename = "@secttype")]
    pub secttype: String,
    pub pstext: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegAddress {
    #[serde(rename = "@external_access", deserialize_with = "titlecase_bool")]
    pub external_access: bool,
    #[serde(rename = "@mem_map_access", deserialize_with = "titlecase_bool")]
    pub mem_map_access: bool,
    #[serde(
        rename = "@block_access",
        deserialize_with = "titlecase_bool_option",
        default
    )]
    pub block_access: Option<bool>,
    #[serde(
        rename = "@memory_access",
        deserialize_with = "titlecase_bool_option",
        default
    )]
    pub memory_access: Option<bool>,
    #[serde(rename = "@table_id")]
    pub table_id: Option<String>,
    #[serde(rename = "@power_domain")]
    pub power_domain: Option<String>,
    pub reg_component: Option<String>,
    pub reg_frame: Option<String>,
    pub reg_offset: RegOffset,
    pub reg_instance: Option<String>,
    pub reg_access: RegAccess,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegOffset {
    pub hexnumber: String,
}

impl RegOffset {
    pub fn parse_hex(&self) -> Option<u64> {
        let (prefix, rest) = self.hexnumber.split_at_checked(2)?;
        if prefix == "0x" {
            u64::from_str_radix(rest, 16).ok()
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegAccess {
    pub reg_access_state: Vec<RegAccessState>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegAccessState {
    pub reg_access_level: Option<String>,
    pub reg_access_type: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ArchVariants {
    #[serde(default)]
    pub arch_variant: Vec<ArchVariant>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct ArchVariant {
    #[serde(rename = "@name")]
    pub name: String,
}

fn titlecase_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    match String::deserialize(deserializer)?.as_ref() {
        "True" => Ok(true),
        "False" => Ok(false),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Str(other),
            &"True or False",
        )),
    }
}

fn titlecase_bool_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    match <Option<String>>::deserialize(deserializer)?.as_deref() {
        Some("True") => Ok(Some(true)),
        Some("False") => Ok(Some(false)),
        None => Ok(None),
        Some(other) => Err(serde::de::Error::invalid_value(
            Unexpected::Str(other),
            &"True or False",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::de;
    use std::{
        fs::{File, read_dir},
        io::BufReader,
    };

    #[test]
    fn parse_reg_purpose() {
        let reg_purpose: RegPurpose = de::from_str(
            "<reg_purpose><purpose_text><para>foo</para></purpose_text></reg_purpose>",
        )
        .unwrap();
        assert_eq!(
            reg_purpose,
            RegPurpose {
                purpose_text: vec![Text {
                    text: vec![TextEntry::Para(Para {})]
                }]
            }
        );
    }

    #[test]
    fn parse_reg_condition_string() {
        let reg_condition: RegCondition =
            de::from_str("<reg_condition>foo</reg_condition>").unwrap();
        assert_eq!(
            reg_condition,
            RegCondition {
                otherwise: None,
                condition: vec![TextEntry::String("foo".to_string())]
            }
        );
    }

    #[test]
    fn parse_reg_condition_para() {
        let reg_condition: RegCondition =
            de::from_str("<reg_condition><para>foo</para></reg_condition>").unwrap();
        assert_eq!(
            reg_condition,
            RegCondition {
                otherwise: None,
                condition: vec![TextEntry::Para(Para {})]
            }
        );
    }

    #[test]
    fn parse_field_description_empty() {
        let field_description: FieldDescription =
            de::from_str("<field_description order=\"before\"/>").unwrap();
        assert_eq!(
            field_description,
            FieldDescription {
                order: Order::Before,
                description: vec![],
            }
        );
    }

    #[test]
    fn parse_field_description_para() {
        let field_description: FieldDescription = de::from_str(
            "<field_description order=\"before\"><para>foo</para></field_description>",
        )
        .unwrap();
        assert_eq!(
            field_description,
            FieldDescription {
                order: Order::Before,
                description: vec![TextEntry::Para(Para {})],
            }
        );
    }

    #[test]
    fn parse_reg_reset_value_empty() {
        let reg_reset_value: RegResetValue =
            de::from_str("<reg_reset_value></reg_reset_value>").unwrap();
        assert_eq!(
            reg_reset_value,
            RegResetValue {
                reg_reset_limited_to_el: vec![],
                reg_reset_special_text: None
            }
        );
    }

    #[test]
    fn parse_reg_attributes() {
        let reg_attributes: RegAttributes = de::from_str(
            "<reg_attributes><attributes_text><para>foo</para></attributes_text></reg_attributes>",
        )
        .unwrap();
        assert_eq!(
            reg_attributes,
            RegAttributes {
                attributes_text: vec![Text {
                    text: vec![TextEntry::Para(Para {})]
                }]
            }
        );
    }

    #[test]
    fn parse_text_before_fields() {
        let text_before_fields: Text =
            de::from_str("<text_before_fields><para>foo</para></text_before_fields>").unwrap();
        assert_eq!(
            text_before_fields,
            Text {
                text: vec![TextEntry::Para(Para {})]
            }
        );
    }

    #[test]
    fn parse_hexnumber() {
        let reg_offset: RegOffset =
            de::from_str("<reg_offset><hexnumber>0x18</hexnumber></reg_offset>").unwrap();
        assert_eq!(reg_offset.parse_hex(), Some(0x18));
    }

    #[test]
    fn parse_enc() {
        let enc: Enc = de::from_str("<enc n=\"coproc\" v=\"0b1101\"/>").unwrap();
        assert_eq!(enc.n, EncName::Coproc);
        assert_eq!(enc.parse_value(), Some(0b1101));
    }

    #[test]
    fn parse_all() {
        let mut failed = 0;
        let mut succeeded = 0;
        for entry in read_dir("SysReg_xml_A_profile-2025-06/SysReg_xml_A_profile-2025-06").unwrap()
        {
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
                if let Err(e) = de::from_reader::<_, RegisterPage>(BufReader::new(
                    File::open(entry.path()).unwrap(),
                )) {
                    println!("{filename}:");
                    println!("{e}");
                    failed += 1;
                } else {
                    succeeded += 1;
                }
            }
        }
        println!("{succeeded} succeeded");
        assert_eq!(failed, 0);
    }
}
