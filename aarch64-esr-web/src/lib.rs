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

use aarch64_esr_decoder::{decode, Decoded, FieldInfo};
use std::num::ParseIntError;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element};

#[wasm_bindgen]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn decode_esr(esr: &str) -> Result<(), JsValue> {
    match parse_number(esr) {
        Ok(esr) => {
            let decoded = decode(esr);
            match &decoded {
                Ok(decoded) => show_decoded(esr, decoded)?,
                Err(e) => show_error(&e.to_string()),
            }
        }
        Err(_) => show_error("Invalid ESR"),
    }
    Ok(())
}

fn show_error(error: &str) {
    let document = web_sys::window()
        .expect("Couldn't find window")
        .document()
        .expect("Couldn't find document");
    let error_element = document
        .get_element_by_id("error")
        .expect("Couldn't find error element");
    let table = document
        .get_element_by_id("result_table")
        .expect("Couldn't find result table");
    // Clear output table.
    table.set_inner_html("");
    error_element.set_text_content(Some(error));
}

fn show_decoded(esr: u64, decoded: &Decoded) -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("Couldn't find window")
        .document()
        .expect("Couldn't find document");
    let error_element = document
        .get_element_by_id("error")
        .expect("Couldn't find error element");
    let table = document
        .get_element_by_id("result_table")
        .expect("Couldn't find result table");
    // Remove existing contents.
    error_element.set_inner_html("");
    table.set_inner_html("");

    // ESR in hexadecimal
    let row = document.create_element("tr")?;
    row.set_attribute("class", "value")?;
    let esr_hex = format!("{:016x}", esr);
    for digit in esr_hex.chars() {
        let cell = make_cell(&document, Some(&digit.to_string()), 4)?;
        row.append_child(&cell)?;
    }
    table.append_child(&row)?;

    // ESR in binary
    let row = document.create_element("tr")?;
    row.set_attribute("class", "value")?;
    for i in (0..u64::BITS).rev() {
        let bit = esr & (1 << i) != 0;
        let cell = make_cell(&document, Some(if bit { "1" } else { "0" }), 1)?;
        row.append_child(&cell)?;
    }
    table.append_child(&row)?;

    // Top-level description
    let row = document.create_element("tr")?;
    row.set_attribute("class", "description")?;
    let cell = make_cell(&document, decoded.description.as_deref(), 64)?;
    row.append_child(&cell)?;
    table.append_child(&row)?;

    // Top-level field names and values
    let row = document.create_element("tr")?;
    row.set_attribute("class", "name")?;
    let mut last = 64;
    add_field_cells(&document, &row, &decoded.fields, &mut last, |field| {
        Some(field.to_string())
    })?;
    table.append_child(&row)?;

    // Top-level field descriptions
    let row = document.create_element("tr")?;
    row.set_attribute("class", "description")?;
    let mut last = 64;
    add_field_cells(&document, &row, &decoded.fields, &mut last, |field| {
        field
            .decoded
            .as_ref()
            .and_then(|decoded| decoded.description.clone())
    })?;
    table.append_child(&row)?;

    // Second level field names and values
    let row = document.create_element("tr")?;
    row.set_attribute("class", "name")?;
    let mut last = 64;
    for field in &decoded.fields {
        if let Some(field_decoded) = &field.decoded {
            add_field_cells(&document, &row, &field_decoded.fields, &mut last, |field| {
                Some(field.to_string())
            })?;
        }
    }
    table.append_child(&row)?;

    // Second level field descriptions
    let row = document.create_element("tr")?;
    row.set_attribute("class", "description")?;
    let mut last = 64;
    for field in &decoded.fields {
        if let Some(field_decoded) = &field.decoded {
            add_field_cells(&document, &row, &field_decoded.fields, &mut last, |field| {
                field
                    .decoded
                    .as_ref()
                    .and_then(|decoded| decoded.description.clone())
            })?;
        }
    }
    table.append_child(&row)?;

    Ok(())
}

fn make_cell(
    document: &Document,
    contents: Option<&str>,
    colspan: usize,
) -> Result<Element, JsValue> {
    let cell = document.create_element("td")?;
    cell.set_attribute("colspan", &colspan.to_string())?;
    cell.set_text_content(contents);
    Ok(cell)
}

fn add_field_cells<F, S>(
    document: &Document,
    row: &Element,
    fields: &[FieldInfo],
    last: &mut usize,
    get_contents: F,
) -> Result<(), JsValue>
where
    F: Fn(&FieldInfo) -> Option<S>,
    S: Deref<Target = str>,
{
    for field in fields {
        if field.start + field.width != *last {
            // Add a filler
            let cell = make_cell(document, None, *last - field.start - field.width)?;
            row.append_child(&cell)?;
        }
        let cell = make_cell(document, get_contents(field).as_deref(), field.width)?;
        row.append_child(&cell)?;
        *last = field.start;
    }
    Ok(())
}

/// Parse a decimal or hexadecimal number.
fn parse_number(s: &str) -> Result<u64, ParseIntError> {
    if s.starts_with("0x") {
        u64::from_str_radix(&s[2..], 16)
    } else {
        s.parse()
    }
}
