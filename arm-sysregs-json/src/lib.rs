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

//! Types to parse the Arm system register JSON format.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "_type")]
pub enum RegisterEntry {
    Register(Register),
    RegisterArray(RegisterArray),
    RegisterBlock(RegisterBlock),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Register {
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
    pub access_text: Option<String>,
    pub accessors: Vec<Accessor>,
    pub condition: Condition,
    pub configuration: Option<String>,
    pub fieldsets: Vec<Fieldset>,
    pub groups: Option<()>,
    pub instances: Instances,
    pub mapset: Vec<()>,
    pub name: String,
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ExecutionState>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Meta {
    pub license: License,
    pub version: Version,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct License {
    pub copyright: String,
    pub info: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Version {
    pub architecture: String,
    pub build: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub schema: String,
    pub timestamp: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum ExecutionState {
    AArch32,
    AArch64,
    #[serde(rename = "ext")]
    External,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Accessor {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Condition {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Fieldset {
    pub condition: Condition,
    pub description: Description,
    pub display: Option<String>,
    pub name: Option<String>,
    pub values: Vec<Fields>,
    pub width: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Fields {
    pub description: Description,
    pub name: Option<String>,
    pub rangeset: Vec<Range>,
    pub resets: Option<FieldResets>,
    pub values: Option<Values>,
    pub volatile: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Description {
    pub after: Option<String>,
    pub before: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Range {
    pub start: u32,
    pub width: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FieldResets {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Values {
    pub values: Vec<ValueEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "_type")]
pub enum ValueEntry {
    #[serde(rename = "Values.ConditionalValue")]
    ConditionalValue(ConditionalValue),
    #[serde(rename = "Values.EquationValue")]
    EquationValue(EquationValue),
    #[serde(rename = "Values.Group")]
    Group(Group),
    #[serde(rename = "Values.Link")]
    Link(Link),
    #[serde(rename = "Values.NamedValue")]
    NamedValue(NamedValue),
    #[serde(rename = "Values.Value")]
    Value(Value),
    #[serde(rename = "Values.ValueRange")]
    ValueRange(ValueRange),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConditionalValue {
    pub condition: Condition,
    pub meaning: Option<String>,
    pub values: Values,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EquationValue {
    pub meaning: Option<String>,
    pub slice: Vec<Range>,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Group {
    pub meaning: Option<String>,
    pub value: String,
    pub values: Values,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Link {
    pub links: BTreeMap<String, String>,
    pub meaning: Option<String>,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NamedValue {
    pub meaning: Option<String>,
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Value {
    pub meaning: Option<String>,
    pub value: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ValueRange {
    pub end: Value,
    pub meaning: Option<String>,
    pub start: Value,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegisterArray {
    #[serde(rename = "_meta")]
    pub meta: Option<Meta>,
    pub access_text: Option<String>,
    pub accessors: Vec<Accessor>,
    pub condition: Condition,
    pub configuration: Option<String>,
    pub fieldsets: Vec<Fieldset>,
    pub groups: Option<()>,
    pub index_variable: String,
    pub indexes: Vec<Range>,
    pub instances: Instances,
    pub mapset: Vec<()>,
    pub name: String,
    pub purpose: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ExecutionState>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Instances {
    InstanceSet(InstanceSet),
    Bool(bool),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InstanceSet {
    pub values: Vec<Instance>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Instance {
    pub condition: Condition,
    pub instance: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RegisterBlock {
    #[serde(rename = "_meta")]
    pub meta: Meta,
    pub accessors: Vec<Accessor>,
    pub blocks: Vec<RegisterEntry>,
    pub condition: Condition,
    pub default_access: ReadWriteAccess,
    pub mapset: Vec<()>,
    pub name: String,
    pub purpose: Option<String>,
    pub references: References,
    pub size: String,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ReadWriteAccess {
    pub read: String,
    pub write: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct References {}
