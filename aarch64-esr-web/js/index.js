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

import("../pkg").then((wasm) => {
  wasm.init();

  const esr = document.getElementById("esr");
  if (esr != null) {
    esr.oninput = () => {
      const value = esr.value.trim();
      if (value.length > 0) {
        wasm.decode_esr(value);
      }
      window.location.hash = value;
    };

    if (window.location.hash) {
      const value = window.location.hash.substring(1).trim();
      esr.value = value;
      if (value.length > 0) {
        wasm.decode_esr(value);
      }
    }
  }

  const midr = document.getElementById("midr");
  if (midr != null) {
    midr.oninput = () => {
      const value = midr.value.trim();
      if (value.length > 0) {
        wasm.decode_midr(value);
      }
      window.location.hash = value;
    };

    if (window.location.hash) {
      const value = window.location.hash.substring(1).trim();
      midr.value = value;
      if (value.length > 0) {
        wasm.decode_midr(value);
      }
    }
  }

  const smccc = document.getElementById("smccc");
  if (smccc != null) {
    smccc.oninput = () => {
      const value = smccc.value.trim();
      if (value.length > 0) {
        wasm.decode_smccc(value);
      }
      window.location.hash = value;
    };

    if (window.location.hash) {
      const value = window.location.hash.substring(1).trim();
      smccc.value = value;

      if (value.length > 0) {
        wasm.decode_smccc(value);
      }
    }
  }
});
