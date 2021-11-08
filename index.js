import * as wasm from "aarch64-esr-web";

const esr = document.getElementById("esr");
esr.oninput = () => {
    if (esr.value.length > 0) {
        wasm.decode_esr(esr.value)
    }
    window.location.hash = esr.value
};

if (window.location.hash) {
    esr.value = window.location.hash.substring(1)
    wasm.decode_esr(esr.value)
}
