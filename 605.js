"use strict";(self.webpackChunkaarch64_esr_web_www=self.webpackChunkaarch64_esr_web_www||[]).push([[605],{605:(e,n,t)=>{t.a(e,(async(e,_)=>{try{t.r(n),t.d(n,{__wbg_appendChild_bc4a0deae90a5164:()=>r.Ax,__wbg_call_a9ef466721e824f2:()=>r.HG,__wbg_createElement_e4523490bd0ae51d:()=>r.Vv,__wbg_document_d7fa2c739c2b191a:()=>r.rU,__wbg_error_f851667af71bcfc6:()=>r.Xu,__wbg_getElementById_734c4eac4fec5911:()=>r.X6,__wbg_globalThis_05c129bf37fcf1be:()=>r.Bn,__wbg_global_3eca19bb09e9c484:()=>r.KO,__wbg_instanceof_Window_6575cd7f1322f82f:()=>r.ds,__wbg_new_abda76e883ba8a5f:()=>r.V5,__wbg_newnoargs_1ede4bf2ebbaaf43:()=>r.ez,__wbg_self_bf91bf94d9e04084:()=>r.EI,__wbg_setAttribute_2a8f647a8d92c712:()=>r.o2,__wbg_set_wasm:()=>r.lI,__wbg_setinnerHTML_559d45055154f1d8:()=>r.Dc,__wbg_settextContent_f9c4b60e6c009ea2:()=>r.g8,__wbg_stack_658279fe44541cf6:()=>r.u$,__wbg_window_52dd9f07d03fd5f8:()=>r.Mt,__wbindgen_init_externref_table:()=>r.bL,__wbindgen_is_undefined:()=>r.vU,__wbindgen_throw:()=>r.Qn,decode_esr:()=>r.c2,decode_midr:()=>r.YH,decode_smccc:()=>r.LQ,init:()=>r.Ts});var c=t(650),r=t(903),o=e([c]);c=(o.then?(await o)():o)[0],(0,r.lI)(c),c.__wbindgen_start(),_()}catch(e){_(e)}}))},903:(e,n,t)=>{let _;function c(e){_=e}t.d(n,{Ax:()=>K,Bn:()=>D,Dc:()=>I,EI:()=>H,HG:()=>X,KO:()=>M,LQ:()=>h,Mt:()=>B,Qn:()=>Q,Ts:()=>d,V5:()=>L,Vv:()=>O,X6:()=>E,Xu:()=>A,YH:()=>s,bL:()=>W,c2:()=>g,ds:()=>G,ez:()=>V,g8:()=>C,lI:()=>c,o2:()=>z,rU:()=>T,u$:()=>k,vU:()=>U});let r=new("undefined"==typeof TextDecoder?(0,module.require)("util").TextDecoder:TextDecoder)("utf-8",{ignoreBOM:!0,fatal:!0});r.decode();let o=null;function f(){return null!==o&&0!==o.byteLength||(o=new Uint8Array(_.memory.buffer)),o}function i(e,n){return e>>>=0,r.decode(f().subarray(e,e+n))}function d(){_.init()}let b=0,u=new("undefined"==typeof TextEncoder?(0,module.require)("util").TextEncoder:TextEncoder)("utf-8");const a="function"==typeof u.encodeInto?function(e,n){return u.encodeInto(e,n)}:function(e,n){const t=u.encode(e);return n.set(t),{read:e.length,written:t.length}};function l(e,n,t){if(void 0===t){const t=u.encode(e),_=n(t.length,1)>>>0;return f().subarray(_,_+t.length).set(t),b=t.length,_}let _=e.length,c=n(_,1)>>>0;const r=f();let o=0;for(;o<_;o++){const n=e.charCodeAt(o);if(n>127)break;r[c+o]=n}if(o!==_){0!==o&&(e=e.slice(o)),c=t(c,_,_=o+3*e.length,1)>>>0;const n=f().subarray(c+o,c+_);o+=a(e,n).written,c=t(c,_,o,1)>>>0}return b=o,c}function w(e){const n=_.__wbindgen_export_0.get(e);return _.__externref_table_dealloc(e),n}function g(e){const n=l(e,_.__wbindgen_malloc,_.__wbindgen_realloc),t=b,c=_.decode_esr(n,t);if(c[1])throw w(c[0])}function s(e){const n=l(e,_.__wbindgen_malloc,_.__wbindgen_realloc),t=b,c=_.decode_midr(n,t);if(c[1])throw w(c[0])}function h(e){const n=l(e,_.__wbindgen_malloc,_.__wbindgen_realloc),t=b,c=_.decode_smccc(n,t);if(c[1])throw w(c[0])}let m=null;function x(){return(null===m||!0===m.buffer.detached||void 0===m.buffer.detached&&m.buffer!==_.memory.buffer)&&(m=new DataView(_.memory.buffer)),m}function y(e){const n=_.__externref_table_alloc();return _.__wbindgen_export_0.set(n,e),n}function p(e,n){try{return e.apply(this,n)}catch(e){const n=y(e);_.__wbindgen_exn_store(n)}}function v(e){return null==e}function T(e){const n=e.document;return v(n)?0:y(n)}function E(e,n,t){const _=e.getElementById(i(n,t));return v(_)?0:y(_)}function I(e,n,t){e.innerHTML=i(n,t)}function C(e,n,t){e.textContent=0===n?void 0:i(n,t)}function L(){return new Error}function k(e,n){const t=l(n.stack,_.__wbindgen_malloc,_.__wbindgen_realloc),c=b;x().setInt32(e+4,c,!0),x().setInt32(e+0,t,!0)}function A(e,n){let t,c;try{t=e,c=n,console.error(i(e,n))}finally{_.__wbindgen_free(t,c,1)}}function H(){return p((function(){return self.self}),arguments)}function B(){return p((function(){return window.window}),arguments)}function D(){return p((function(){return globalThis.globalThis}),arguments)}function M(){return p((function(){return global.global}),arguments)}function U(e){return void 0===e}function V(e,n){return new Function(i(e,n))}function X(){return p((function(e,n){return e.call(n)}),arguments)}function Q(e,n){throw new Error(i(e,n))}function O(){return p((function(e,n,t){return e.createElement(i(n,t))}),arguments)}function z(){return p((function(e,n,t,_,c){e.setAttribute(i(n,t),i(_,c))}),arguments)}function G(e){let n;try{n=e instanceof Window}catch(e){n=!1}return n}function K(){return p((function(e,n){return e.appendChild(n)}),arguments)}function W(){const e=_.__wbindgen_export_0,n=e.grow(4);e.set(0,void 0),e.set(n+0,void 0),e.set(n+1,null),e.set(n+2,!0),e.set(n+3,!1)}},650:(e,n,t)=>{var _=t(903);e.exports=t.v(n,e.id,"8ccb6b3bfce8966daf15",{"./index_bg.js":{__wbg_document_d7fa2c739c2b191a:_.rU,__wbg_getElementById_734c4eac4fec5911:_.X6,__wbg_setinnerHTML_559d45055154f1d8:_.Dc,__wbg_settextContent_f9c4b60e6c009ea2:_.g8,__wbg_new_abda76e883ba8a5f:_.V5,__wbg_stack_658279fe44541cf6:_.u$,__wbg_error_f851667af71bcfc6:_.Xu,__wbg_self_bf91bf94d9e04084:_.EI,__wbg_window_52dd9f07d03fd5f8:_.Mt,__wbg_globalThis_05c129bf37fcf1be:_.Bn,__wbg_global_3eca19bb09e9c484:_.KO,__wbindgen_is_undefined:_.vU,__wbg_newnoargs_1ede4bf2ebbaaf43:_.ez,__wbg_call_a9ef466721e824f2:_.HG,__wbindgen_throw:_.Qn,__wbg_createElement_e4523490bd0ae51d:_.Vv,__wbg_setAttribute_2a8f647a8d92c712:_.o2,__wbg_instanceof_Window_6575cd7f1322f82f:_.ds,__wbg_appendChild_bc4a0deae90a5164:_.Ax,__wbindgen_init_externref_table:_.bL}})}}]);
//# sourceMappingURL=605.js.map