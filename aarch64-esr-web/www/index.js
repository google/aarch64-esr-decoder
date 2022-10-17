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

import * as wasm from "aarch64-esr-web";

wasm.init();

const esr = document.getElementById("esr");
if (esr != null) {
  esr.oninput = () => {
    if (esr.value.length > 0) {
      wasm.decode_esr(esr.value);
    }
    window.location.hash = esr.value;
  };

  if (window.location.hash) {
    esr.value = window.location.hash.substring(1);
    wasm.decode_esr(esr.value);
  }
}

const midr = document.getElementById("midr");
if (midr != null) {
  midr.oninput = () => {
    if (midr.value.length > 0) {
      wasm.decode_midr(midr.value);
    }
    window.location.hash = midr.value;
  };

  if (window.location.hash) {
    midr.value = window.location.hash.substring(1);
    wasm.decode_midr(midr.value);
  }
}

const smccc = document.getElementById("smccc");
if (smccc != null) {
  smccc.oninput = () => {
    if (smccc.value.length > 0) {
      wasm.decode_smccc(smccc.value);
    }
    window.location.hash = smccc.value;
  };

  if (window.location.hash) {
    smccc.value = window.location.hash.substring(1);
    wasm.decode_smccc(smccc.value);
  }
}
