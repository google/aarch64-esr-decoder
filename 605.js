"use strict";(self.webpackChunkaarch64_esr_web_www=self.webpackChunkaarch64_esr_web_www||[]).push([[605],{605:(n,e,t)=>{t.a(n,(async(n,_)=>{try{t.r(e),t.d(e,{__wbg_appendChild_ac45d1abddf1b89b:()=>c.am,__wbg_call_1084a111329e68ce:()=>c.tM,__wbg_createElement_5921e9eb06b9ec89:()=>c.HG,__wbg_document_8554450897a855b9:()=>c.Ne,__wbg_error_f851667af71bcfc6:()=>c.Xu,__wbg_getElementById_f56c8e6a15a6926d:()=>c.P3,__wbg_globalThis_86b222e13bdf32ed:()=>c.tn,__wbg_global_e5a3fe56f8be9485:()=>c.jF,__wbg_instanceof_Window_5012736c80a01584:()=>c.uw,__wbg_new_abda76e883ba8a5f:()=>c.V5,__wbg_newnoargs_76313bd6ff35d0f2:()=>c.xN,__wbg_self_3093d5d1f7bcb682:()=>c.KN,__wbg_setAttribute_d5540a19be09f8dc:()=>c.rA,__wbg_set_wasm:()=>c.lI,__wbg_setinnerHTML_ea7e3c6a3c4790c6:()=>c.im,__wbg_settextContent_cd38ea7d4e0f7260:()=>c.ap,__wbg_stack_658279fe44541cf6:()=>c.u$,__wbg_window_3bcfc4d31bc012f8:()=>c.O9,__wbindgen_is_undefined:()=>c.vU,__wbindgen_object_clone_ref:()=>c.BZ,__wbindgen_object_drop_ref:()=>c.bk,__wbindgen_throw:()=>c.Qn,decode_esr:()=>c.c2,decode_midr:()=>c.YH,decode_smccc:()=>c.LQ,init:()=>c.Ts});var r=t(650),c=t(903),o=n([r]);r=(o.then?(await o)():o)[0],(0,c.lI)(r),_()}catch(n){_(n)}}))},903:(n,e,t)=>{let _;function r(n){_=n}t.d(e,{BZ:()=>W,HG:()=>K,KN:()=>L,LQ:()=>v,Ne:()=>E,O9:()=>Q,P3:()=>C,Qn:()=>G,Ts:()=>w,V5:()=>H,Xu:()=>M,YH:()=>I,am:()=>Z,ap:()=>j,bk:()=>A,c2:()=>k,im:()=>N,jF:()=>F,lI:()=>r,rA:()=>P,tM:()=>V,tn:()=>D,u$:()=>B,uw:()=>X,vU:()=>O,xN:()=>U});const c=new Array(128).fill(void 0);function o(n){return c[n]}c.push(void 0,null,!0,!1);let i=c.length;function d(n){const e=o(n);return function(n){n<132||(c[n]=i,i=n)}(n),e}let f=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});f.decode();let b=null;function u(){return null!==b&&0!==b.byteLength||(b=new Uint8Array(_.memory.buffer)),b}function a(n,e){return n>>>=0,f.decode(u().subarray(n,n+e))}function l(n){i===c.length&&c.push(c.length+1);const e=i;return i=c[e],c[e]=n,e}function w(){_.init()}let g=0,s=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof s.encodeInto?function(n,e){return s.encodeInto(n,e)}:function(n,e){const t=s.encode(n);return e.set(t),{read:n.length,written:t.length}};function m(n,e,t){if(void 0===t){const t=s.encode(n),_=e(t.length,1)>>>0;return u().subarray(_,_+t.length).set(t),g=t.length,_}let _=n.length,r=e(_,1)>>>0;const c=u();let o=0;for(;o<_;o++){const e=n.charCodeAt(o);if(e>127)break;c[r+o]=e}if(o!==_){0!==o&&(n=n.slice(o)),r=t(r,_,_=o+3*n.length,1)>>>0;const e=u().subarray(r+o,r+_);o+=h(n,e).written,r=t(r,_,o,1)>>>0}return g=o,r}let p=null;function y(){return(null===p||!0===p.buffer.detached||void 0===p.buffer.detached&&p.buffer!==_.memory.buffer)&&(p=new DataView(_.memory.buffer)),p}function k(n){try{const t=_.__wbindgen_add_to_stack_pointer(-16),r=m(n,_.__wbindgen_malloc,_.__wbindgen_realloc),c=g;_.decode_esr(t,r,c);var e=y().getInt32(t+0,!0);if(y().getInt32(t+4,!0))throw d(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function I(n){try{const t=_.__wbindgen_add_to_stack_pointer(-16),r=m(n,_.__wbindgen_malloc,_.__wbindgen_realloc),c=g;_.decode_midr(t,r,c);var e=y().getInt32(t+0,!0);if(y().getInt32(t+4,!0))throw d(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function v(n){try{const t=_.__wbindgen_add_to_stack_pointer(-16),r=m(n,_.__wbindgen_malloc,_.__wbindgen_realloc),c=g;_.decode_smccc(t,r,c);var e=y().getInt32(t+0,!0);if(y().getInt32(t+4,!0))throw d(e)}finally{_.__wbindgen_add_to_stack_pointer(16)}}function x(n,e){try{return n.apply(this,e)}catch(n){_.__wbindgen_exn_store(l(n))}}function T(n){return null==n}function E(n){const e=o(n).document;return T(e)?0:l(e)}function A(n){d(n)}function C(n,e,t){const _=o(n).getElementById(a(e,t));return T(_)?0:l(_)}function N(n,e,t){o(n).innerHTML=a(e,t)}function j(n,e,t){o(n).textContent=0===e?void 0:a(e,t)}function H(){return l(new Error)}function B(n,e){const t=m(o(e).stack,_.__wbindgen_malloc,_.__wbindgen_realloc),r=g;y().setInt32(n+4,r,!0),y().setInt32(n+0,t,!0)}function M(n,e){let t,r;try{t=n,r=e,console.error(a(n,e))}finally{_.__wbindgen_free(t,r,1)}}function L(){return x((function(){return l(self.self)}),arguments)}function Q(){return x((function(){return l(window.window)}),arguments)}function D(){return x((function(){return l(globalThis.globalThis)}),arguments)}function F(){return x((function(){return l(global.global)}),arguments)}function O(n){return void 0===o(n)}function U(n,e){return l(new Function(a(n,e)))}function V(){return x((function(n,e){return l(o(n).call(o(e)))}),arguments)}function G(n,e){throw new Error(a(n,e))}function K(){return x((function(n,e,t){return l(o(n).createElement(a(e,t)))}),arguments)}function P(){return x((function(n,e,t,_,r){o(n).setAttribute(a(e,t),a(_,r))}),arguments)}function W(n){return l(o(n))}function X(n){let e;try{e=o(n)instanceof Window}catch(n){e=!1}return e}function Z(){return x((function(n,e){return l(o(n).appendChild(o(e)))}),arguments)}},650:(n,e,t)=>{var _=t(903);n.exports=t.v(e,n.id,"4a7cda32fac22df7b28e",{"./index_bg.js":{__wbg_document_8554450897a855b9:_.Ne,__wbindgen_object_drop_ref:_.bk,__wbg_getElementById_f56c8e6a15a6926d:_.P3,__wbg_setinnerHTML_ea7e3c6a3c4790c6:_.im,__wbg_settextContent_cd38ea7d4e0f7260:_.ap,__wbg_new_abda76e883ba8a5f:_.V5,__wbg_stack_658279fe44541cf6:_.u$,__wbg_error_f851667af71bcfc6:_.Xu,__wbg_self_3093d5d1f7bcb682:_.KN,__wbg_window_3bcfc4d31bc012f8:_.O9,__wbg_globalThis_86b222e13bdf32ed:_.tn,__wbg_global_e5a3fe56f8be9485:_.jF,__wbindgen_is_undefined:_.vU,__wbg_newnoargs_76313bd6ff35d0f2:_.xN,__wbg_call_1084a111329e68ce:_.tM,__wbindgen_throw:_.Qn,__wbg_createElement_5921e9eb06b9ec89:_.HG,__wbg_setAttribute_d5540a19be09f8dc:_.rA,__wbindgen_object_clone_ref:_.BZ,__wbg_instanceof_Window_5012736c80a01584:_.uw,__wbg_appendChild_ac45d1abddf1b89b:_.am}})}}]);
//# sourceMappingURL=605.js.map