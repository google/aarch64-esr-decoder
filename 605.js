"use strict";(self.webpackChunkaarch64_esr_web_www=self.webpackChunkaarch64_esr_web_www||[]).push([[605],{605:(n,e,_)=>{_.a(n,(async(n,t)=>{try{_.r(e),_.d(e,{__wbg_appendChild_51339d4cde00ee22:()=>c.SV,__wbg_call_cb65541d95d71282:()=>c.no,__wbg_createElement_4891554b28d3388b:()=>c.jB,__wbg_document_f7ace2b956f30a4f:()=>c.aL,__wbg_error_f851667af71bcfc6:()=>c.Xu,__wbg_getElementById_cc0e0d931b0d9a28:()=>c.J$,__wbg_globalThis_1d39714405582d3c:()=>c.xT,__wbg_global_651f05c6a0944d1c:()=>c.Ks,__wbg_instanceof_Window_9029196b662bc42a:()=>c.as,__wbg_new_abda76e883ba8a5f:()=>c.V5,__wbg_newnoargs_581967eacc0e2604:()=>c._h,__wbg_self_1ff1d729e9aae938:()=>c._m,__wbg_setAttribute_e7e80b478b7b8b2f:()=>c.gY,__wbg_set_wasm:()=>c.lI,__wbg_setinnerHTML_b089587252408b67:()=>c.Hz,__wbg_settextContent_28d80502cf08bde7:()=>c.rd,__wbg_stack_658279fe44541cf6:()=>c.u$,__wbg_window_5f4faef6c12b79ec:()=>c.eb,__wbindgen_is_undefined:()=>c.vU,__wbindgen_object_clone_ref:()=>c.BZ,__wbindgen_object_drop_ref:()=>c.bk,__wbindgen_throw:()=>c.Qn,decode_esr:()=>c.c2,decode_midr:()=>c.YH,decode_smccc:()=>c.LQ,init:()=>c.Ts});var r=_(650),c=_(903),o=n([r]);r=(o.then?(await o)():o)[0],(0,c.lI)(r),t()}catch(n){t(n)}}))},903:(n,e,_)=>{let t;function r(n){t=n}_.d(e,{BZ:()=>X,Hz:()=>I,J$:()=>C,Ks:()=>D,LQ:()=>x,Qn:()=>W,SV:()=>V,Ts:()=>w,V5:()=>$,Xu:()=>Y,YH:()=>k,_h:()=>K,_m:()=>M,aL:()=>B,as:()=>Z,bk:()=>L,c2:()=>T,eb:()=>U,gY:()=>A,jB:()=>H,lI:()=>r,no:()=>S,rd:()=>j,u$:()=>Q,vU:()=>J,xT:()=>z});const c=new Array(128).fill(void 0);function o(n){return c[n]}c.push(void 0,null,!0,!1);let i=c.length;function d(n){const e=o(n);return function(n){n<132||(c[n]=i,i=n)}(n),e}let b=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});b.decode();let f=null;function u(){return null!==f&&0!==f.byteLength||(f=new Uint8Array(t.memory.buffer)),f}function a(n,e){return n>>>=0,b.decode(u().subarray(n,n+e))}function l(n){i===c.length&&c.push(c.length+1);const e=i;return i=c[e],c[e]=n,e}function w(){t.init()}let g=0,s=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const h="function"==typeof s.encodeInto?function(n,e){return s.encodeInto(n,e)}:function(n,e){const _=s.encode(n);return e.set(_),{read:n.length,written:_.length}};function y(n,e,_){if(void 0===_){const _=s.encode(n),t=e(_.length,1)>>>0;return u().subarray(t,t+_.length).set(_),g=_.length,t}let t=n.length,r=e(t,1)>>>0;const c=u();let o=0;for(;o<t;o++){const e=n.charCodeAt(o);if(e>127)break;c[r+o]=e}if(o!==t){0!==o&&(n=n.slice(o)),r=_(r,t,t=o+3*n.length,1)>>>0;const e=u().subarray(r+o,r+t);o+=h(n,e).written}return g=o,r}let m=null;function p(){return null!==m&&0!==m.byteLength||(m=new Int32Array(t.memory.buffer)),m}function T(n){try{const _=t.__wbindgen_add_to_stack_pointer(-16),r=y(n,t.__wbindgen_malloc,t.__wbindgen_realloc),c=g;t.decode_esr(_,r,c);var e=p()[_/4+0];if(p()[_/4+1])throw d(e)}finally{t.__wbindgen_add_to_stack_pointer(16)}}function k(n){try{const _=t.__wbindgen_add_to_stack_pointer(-16),r=y(n,t.__wbindgen_malloc,t.__wbindgen_realloc),c=g;t.decode_midr(_,r,c);var e=p()[_/4+0];if(p()[_/4+1])throw d(e)}finally{t.__wbindgen_add_to_stack_pointer(16)}}function x(n){try{const _=t.__wbindgen_add_to_stack_pointer(-16),r=y(n,t.__wbindgen_malloc,t.__wbindgen_realloc),c=g;t.decode_smccc(_,r,c);var e=p()[_/4+0];if(p()[_/4+1])throw d(e)}finally{t.__wbindgen_add_to_stack_pointer(16)}}function v(n,e){try{return n.apply(this,e)}catch(n){t.__wbindgen_exn_store(l(n))}}function E(n){return null==n}function B(n){const e=o(n).document;return E(e)?0:l(e)}function L(n){d(n)}function C(n,e,_){const t=o(n).getElementById(a(e,_));return E(t)?0:l(t)}function I(n,e,_){o(n).innerHTML=a(e,_)}function j(n,e,_){o(n).textContent=0===e?void 0:a(e,_)}function H(){return v((function(n,e,_){return l(o(n).createElement(a(e,_)))}),arguments)}function A(){return v((function(n,e,_,t,r){o(n).setAttribute(a(e,_),a(t,r))}),arguments)}function V(){return v((function(n,e){return l(o(n).appendChild(o(e)))}),arguments)}function $(){return l(new Error)}function Q(n,e){const _=y(o(e).stack,t.__wbindgen_malloc,t.__wbindgen_realloc),r=g;p()[n/4+1]=r,p()[n/4+0]=_}function Y(n,e){let _,r;try{_=n,r=e,console.error(a(n,e))}finally{t.__wbindgen_free(_,r,1)}}function M(){return v((function(){return l(self.self)}),arguments)}function U(){return v((function(){return l(window.window)}),arguments)}function z(){return v((function(){return l(globalThis.globalThis)}),arguments)}function D(){return v((function(){return l(global.global)}),arguments)}function J(n){return void 0===o(n)}function K(n,e){return l(new Function(a(n,e)))}function S(){return v((function(n,e){return l(o(n).call(o(e)))}),arguments)}function W(n,e){throw new Error(a(n,e))}function X(n){return l(o(n))}function Z(n){let e;try{e=o(n)instanceof Window}catch{e=!1}return e}},650:(n,e,_)=>{var t=_(903);n.exports=_.v(e,n.id,"9436708a8069ecf618f4",{"./index_bg.js":{__wbg_document_f7ace2b956f30a4f:t.aL,__wbindgen_object_drop_ref:t.bk,__wbg_getElementById_cc0e0d931b0d9a28:t.J$,__wbg_setinnerHTML_b089587252408b67:t.Hz,__wbg_settextContent_28d80502cf08bde7:t.rd,__wbg_createElement_4891554b28d3388b:t.jB,__wbg_setAttribute_e7e80b478b7b8b2f:t.gY,__wbg_appendChild_51339d4cde00ee22:t.SV,__wbg_new_abda76e883ba8a5f:t.V5,__wbg_stack_658279fe44541cf6:t.u$,__wbg_error_f851667af71bcfc6:t.Xu,__wbg_self_1ff1d729e9aae938:t._m,__wbg_window_5f4faef6c12b79ec:t.eb,__wbg_globalThis_1d39714405582d3c:t.xT,__wbg_global_651f05c6a0944d1c:t.Ks,__wbindgen_is_undefined:t.vU,__wbg_newnoargs_581967eacc0e2604:t._h,__wbg_call_cb65541d95d71282:t.no,__wbindgen_throw:t.Qn,__wbindgen_object_clone_ref:t.BZ,__wbg_instanceof_Window_9029196b662bc42a:t.as}})}}]);
//# sourceMappingURL=605.js.map