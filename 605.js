"use strict";(self.webpackChunkaarch64_esr_web_www=self.webpackChunkaarch64_esr_web_www||[]).push([[605],{605:(e,n,t)=>{t.a(e,(async(e,_)=>{try{t.r(n),t.d(n,{__wbg_appendChild_d22bc7af6b96b3f1:()=>r.f5,__wbg_call_b0d8e36992d9900d:()=>r.Jt,__wbg_createElement_89923fcb809656b7:()=>r.gU,__wbg_document_f11bc4f7c03e1745:()=>r.gF,__wbg_error_7534b8e9a36f1ab4:()=>r.WY,__wbg_getElementById_dcc9f1f3cfdca0bc:()=>r.cD,__wbg_instanceof_Window_d2514c6a7ee7ba60:()=>r.Lc,__wbg_new_8a6f238a6ece86ea:()=>r.$P,__wbg_newnoargs_fd9e4bf8be2bc16d:()=>r.My,__wbg_setAttribute_148e0e65e20e5f27:()=>r.p8,__wbg_set_wasm:()=>r.lI,__wbg_setinnerHTML_2d75307ba8832258:()=>r.P6,__wbg_settextContent_0eab7fce6c07d5c9:()=>r.N8,__wbg_stack_0ed75d68575b0f3c:()=>r.x$,__wbg_static_accessor_GLOBAL_0be7472e492ad3e3:()=>r.Y6,__wbg_static_accessor_GLOBAL_THIS_1a6eb482d12c9bfb:()=>r.In,__wbg_static_accessor_SELF_1dc398a895c82351:()=>r.dZ,__wbg_static_accessor_WINDOW_ae1c80c7eea8d64a:()=>r.EP,__wbindgen_init_externref_table:()=>r.bL,__wbindgen_is_undefined:()=>r.vU,__wbindgen_throw:()=>r.Qn,decode_esr:()=>r.c2,decode_midr:()=>r.YH,decode_smccc:()=>r.LQ,init:()=>r.Ts});var c=t(650),r=t(903),o=e([c]);c=(o.then?(await o)():o)[0],(0,r.lI)(c),c.__wbindgen_start(),_()}catch(e){_(e)}}))},903:(e,n,t)=>{let _;function c(e){_=e}function r(e){const n=_.__externref_table_alloc();return _.__wbindgen_export_2.set(n,e),n}function o(e,n){try{return e.apply(this,n)}catch(e){const n=r(e);_.__wbindgen_exn_store(n)}}t.d(n,{$P:()=>D,EP:()=>F,In:()=>O,Jt:()=>E,LQ:()=>L,Lc:()=>C,My:()=>P,N8:()=>Y,P6:()=>B,Qn:()=>Q,Ts:()=>y,WY:()=>W,Y6:()=>M,YH:()=>x,bL:()=>$,c2:()=>p,cD:()=>A,dZ:()=>U,f5:()=>I,gF:()=>v,gU:()=>T,lI:()=>c,p8:()=>k,vU:()=>N,x$:()=>H});let d=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});d.decode();let i=null;function b(){return null!==i&&0!==i.byteLength||(i=new Uint8Array(_.memory.buffer)),i}function f(e,n){return e>>>=0,d.decode(b().subarray(e,e+n))}function a(e){return null==e}let u=0,s=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const l="function"==typeof s.encodeInto?function(e,n){return s.encodeInto(e,n)}:function(e,n){const t=s.encode(e);return n.set(t),{read:e.length,written:t.length}};function w(e,n,t){if(void 0===t){const t=s.encode(e),_=n(t.length,1)>>>0;return b().subarray(_,_+t.length).set(t),u=t.length,_}let _=e.length,c=n(_,1)>>>0;const r=b();let o=0;for(;o<_;o++){const n=e.charCodeAt(o);if(n>127)break;r[c+o]=n}if(o!==_){0!==o&&(e=e.slice(o)),c=t(c,_,_=o+3*e.length,1)>>>0;const n=b().subarray(c+o,c+_);o+=l(e,n).written,c=t(c,_,o,1)>>>0}return u=o,c}let g=null;function h(){return(null===g||!0===g.buffer.detached||void 0===g.buffer.detached&&g.buffer!==_.memory.buffer)&&(g=new DataView(_.memory.buffer)),g}function y(){_.init()}function m(e){const n=_.__wbindgen_export_2.get(e);return _.__externref_table_dealloc(e),n}function p(e){const n=w(e,_.__wbindgen_malloc,_.__wbindgen_realloc),t=u,c=_.decode_esr(n,t);if(c[1])throw m(c[0])}function x(e){const n=w(e,_.__wbindgen_malloc,_.__wbindgen_realloc),t=u,c=_.decode_midr(n,t);if(c[1])throw m(c[0])}function L(e){const n=w(e,_.__wbindgen_malloc,_.__wbindgen_realloc),t=u,c=_.decode_smccc(n,t);if(c[1])throw m(c[0])}function I(){return o((function(e,n){return e.appendChild(n)}),arguments)}function E(){return o((function(e,n){return e.call(n)}),arguments)}function T(){return o((function(e,n,t){return e.createElement(f(n,t))}),arguments)}function v(e){const n=e.document;return a(n)?0:r(n)}function W(e,n){let t,c;try{t=e,c=n,console.error(f(e,n))}finally{_.__wbindgen_free(t,c,1)}}function A(e,n,t){const _=e.getElementById(f(n,t));return a(_)?0:r(_)}function C(e){let n;try{n=e instanceof Window}catch(e){n=!1}return n}function D(){return new Error}function P(e,n){return new Function(f(e,n))}function k(){return o((function(e,n,t,_,c){e.setAttribute(f(n,t),f(_,c))}),arguments)}function B(e,n,t){e.innerHTML=f(n,t)}function Y(e,n,t){e.textContent=0===n?void 0:f(n,t)}function H(e,n){const t=w(n.stack,_.__wbindgen_malloc,_.__wbindgen_realloc),c=u;h().setInt32(e+4,c,!0),h().setInt32(e+0,t,!0)}function M(){const e="undefined"==typeof global?null:global;return a(e)?0:r(e)}function O(){const e="undefined"==typeof globalThis?null:globalThis;return a(e)?0:r(e)}function U(){const e="undefined"==typeof self?null:self;return a(e)?0:r(e)}function F(){const e="undefined"==typeof window?null:window;return a(e)?0:r(e)}function $(){const e=_.__wbindgen_export_2,n=e.grow(4);e.set(0,void 0),e.set(n+0,void 0),e.set(n+1,null),e.set(n+2,!0),e.set(n+3,!1)}function N(e){return void 0===e}function Q(e,n){throw new Error(f(e,n))}},650:(e,n,t)=>{var _=t(903);e.exports=t.v(n,e.id,"3b2dc7743ad9ff7f5258",{"./index_bg.js":{__wbg_document_f11bc4f7c03e1745:_.gF,__wbg_getElementById_dcc9f1f3cfdca0bc:_.cD,__wbg_setinnerHTML_2d75307ba8832258:_.P6,__wbg_settextContent_0eab7fce6c07d5c9:_.N8,__wbg_createElement_89923fcb809656b7:_.gU,__wbg_setAttribute_148e0e65e20e5f27:_.p8,__wbg_appendChild_d22bc7af6b96b3f1:_.f5,__wbg_new_8a6f238a6ece86ea:_.$P,__wbg_stack_0ed75d68575b0f3c:_.x$,__wbg_error_7534b8e9a36f1ab4:_.WY,__wbg_static_accessor_WINDOW_ae1c80c7eea8d64a:_.EP,__wbg_static_accessor_GLOBAL_0be7472e492ad3e3:_.Y6,__wbg_static_accessor_SELF_1dc398a895c82351:_.dZ,__wbindgen_is_undefined:_.vU,__wbg_newnoargs_fd9e4bf8be2bc16d:_.My,__wbg_call_b0d8e36992d9900d:_.Jt,__wbg_static_accessor_GLOBAL_THIS_1a6eb482d12c9bfb:_.In,__wbindgen_throw:_.Qn,__wbg_instanceof_Window_d2514c6a7ee7ba60:_.Lc,__wbindgen_init_externref_table:_.bL}})}}]);
//# sourceMappingURL=605.js.map