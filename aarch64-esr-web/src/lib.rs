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

use aarch64_esr_decoder::{decode, parse_number, DecodeError, FieldInfo};
use std::convert::TryFrom;
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
            show_decoded(esr, decoded, u64::BITS)?;
        }
        Err(_) => show_error("Invalid ESR"),
    }
    Ok(())
}

#[wasm_bindgen]
pub fn decode_midr(midr: &str) -> Result<(), JsValue> {
    match parse_number(midr) {
        Ok(midr) => {
            let decoded = aarch64_esr_decoder::decode_midr(midr);
            show_decoded(midr, decoded, u64::BITS)?;
        }
        Err(_) => show_error("Invalid MIDR"),
    }
    Ok(())
}

#[wasm_bindgen]
pub fn decode_smccc(fn_id: &str) -> Result<(), JsValue> {
    match parse_number(fn_id) {
        Ok(fn_id) => {
            let decoded = aarch64_esr_decoder::decode_smccc(fn_id);
            show_decoded(fn_id, decoded, u32::BITS)?;
        }
        Err(_) => show_error("Invalid SMCCC Function ID"),
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

fn show_decoded(
    esr: u64,
    decoded: Result<Vec<FieldInfo>, DecodeError>,
    nr_bit: u32,
) -> Result<(), JsValue> {
    assert!(nr_bit <= u64::BITS);
    let nr_bit = usize::try_from(nr_bit).unwrap();
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
    // Loop over number of chars of interest only
    let nr_nib = (nr_bit + 3) / 4;
    for digit in esr_hex.chars().skip(16 - nr_nib) {
        let cell = make_cell(&document, Some(&digit.to_string()), None, 4)?;
        row.append_child(&cell)?;
    }
    table.append_child(&row)?;

    // ESR in binary
    let row = document.create_element("tr")?;
    row.set_attribute("class", "value")?;
    for i in (0..nr_bit).rev() {
        let bit = esr & (1 << i) != 0;
        let cell = make_cell(&document, Some(if bit { "1" } else { "0" }), None, 1)?;
        row.append_child(&cell)?;
    }
    table.append_child(&row)?;

    match decoded {
        Ok(fields) => {
            // Top-level field names and values
            let row = document.create_element("tr")?;
            row.set_attribute("class", "name")?;
            let mut last = nr_bit;
            add_field_cells(
                &document,
                &row,
                &fields,
                &mut last,
                |field| Some(field.to_string()),
                |field| field.long_name,
            )?;
            table.append_child(&row)?;

            // Top-level field descriptions
            let row = document.create_element("tr")?;
            row.set_attribute("class", "description")?;
            let mut last = nr_bit;
            add_field_cells(
                &document,
                &row,
                &fields,
                &mut last,
                |field| field.description.clone(),
                |_| None,
            )?;
            table.append_child(&row)?;

            // Second level field names and values
            let row = document.create_element("tr")?;
            row.set_attribute("class", "name")?;
            let mut last = nr_bit;
            for field in &fields {
                add_field_cells(
                    &document,
                    &row,
                    &field.subfields,
                    &mut last,
                    |field| Some(field.to_string()),
                    |field| field.long_name,
                )?;
            }
            table.append_child(&row)?;

            // Second level field descriptions
            let row = document.create_element("tr")?;
            row.set_attribute("class", "description")?;
            let mut last = nr_bit;
            for field in &fields {
                add_field_cells(
                    &document,
                    &row,
                    &field.subfields,
                    &mut last,
                    |field| field.description.clone(),
                    |_| None,
                )?;
            }
            table.append_child(&row)?;
        }
        Err(e) => error_element.set_text_content(Some(&e.to_string())),
    }

    Ok(())
}

fn make_cell(
    document: &Document,
    contents: Option<&str>,
    hover_title: Option<&str>,
    colspan: usize,
) -> Result<Element, JsValue> {
    let cell = document.create_element("td")?;
    cell.set_attribute("colspan", &colspan.to_string())?;
    if let Some(title) = hover_title {
        let abbr = document.create_element("abbr")?;
        abbr.set_attribute("title", title)?;
        abbr.set_text_content(contents);
        cell.append_child(&abbr)?;
    } else {
        cell.set_text_content(contents);
    }
    Ok(cell)
}

fn add_field_cells<F, G, S>(
    document: &Document,
    row: &Element,
    fields: &[FieldInfo],
    last: &mut usize,
    get_contents: F,
    get_hover_title: G,
) -> Result<(), JsValue>
where
    F: Fn(&FieldInfo) -> Option<S>,
    G: Fn(&FieldInfo) -> Option<&str>,
    S: Deref<Target = str>,
{
    for field in fields {
        if field.start + field.width != *last {
            // Add a filler
            let cell = make_cell(document, None, None, *last - field.start - field.width)?;
            row.append_child(&cell)?;
        }
        let cell = make_cell(
            document,
            get_contents(field).as_deref(),
            get_hover_title(field).as_deref(),
            field.width,
        )?;
        row.append_child(&cell)?;
        *last = field.start;
    }
    Ok(())
}
