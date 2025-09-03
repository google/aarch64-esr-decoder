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

use quick_xml::de;
use serde::{Deserialize, Deserializer, de::Unexpected};
use std::io::stdin;

fn main() {
    let register_page: RegisterPage = de::from_reader(stdin().lock()).unwrap();
    println!("{register_page:#?}");
}

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
    pub execution_state: ExecutionState,
    #[serde(rename = "@is_register", deserialize_with = "titlecase_bool")]
    pub is_register: bool,
    #[serde(rename = "@is_internal", deserialize_with = "titlecase_bool")]
    pub is_internal: bool,
    #[serde(rename = "@is_stub_entry", deserialize_with = "titlecase_bool")]
    pub is_stub_entry: bool,
    pub reg_short_name: String,
    pub reg_long_name: String,
    pub reg_condition: RegCondition,
    pub reg_reset_value: String,
    pub reg_mappings: (), // TODO
    pub reg_purpose: RegPurpose,
    pub reg_attributes: RegAttributes,
    pub reg_fieldsets: RegFieldsets,
    pub access_mechanisms: AccessMechanisms,
    pub arch_variants: (), // TODO
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum ExecutionState {
    AArch64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegCondition {
    #[serde(rename = "@otherwise")]
    pub otherwise: String,
    #[serde(rename = "$value")]
    pub condition: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegPurpose {
    pub purpose_text: Text,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegGroups {
    pub reg_group: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegAttributes {
    pub attributes_text: Text,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegFieldsets {
    pub fields: Fields,
    pub reg_fieldset: RegFieldset,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Fields {
    pub text_before_fields: String,
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
    //#[serde(rename = "$value")]
    //pub description: Text, // TODO
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Text {
    pub para: Vec<Para>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct Para {}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Order {
    Before,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct RegFieldset {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessMechanisms {
    pub access_mechanism: Vec<AccessMechanism>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessMechanism {
    #[serde(rename = "@accessor")]
    pub accessor: String,
    #[serde(rename = "@type")]
    pub type_: AccessMechanismType,
    pub encoding: Encoding,
    pub access_permission: AccessPermission,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum AccessMechanismType {
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
    #[serde(rename = "@v", deserialize_with = "binary_u8")]
    pub v: u8,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum EncName {
    #[serde(rename = "op0")]
    Op0,
    #[serde(rename = "op1")]
    Op1,
    CRn,
    CRm,
    #[serde(rename = "op2")]
    Op2,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct AccessPermission {
    pub ps: Ps,
}

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

fn binary_u8<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u8, D::Error> {
    let s = String::deserialize(deserializer)?;
    let (prefix, rest) = s
        .split_at_checked(2)
        .ok_or(serde::de::Error::invalid_value(
            Unexpected::Str(&s),
            &"binary number",
        ))?;
    if prefix != "0b" {
        return Err(serde::de::Error::invalid_value(
            Unexpected::Str(&s),
            &"binary number",
        ));
    }
    u8::from_str_radix(rest, 2)
        .map_err(|_| serde::de::Error::invalid_value(Unexpected::Str(&s), &"binary number"))
}
