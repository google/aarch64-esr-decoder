(()=>{var e,t,r,n,o,a,i={},s={};function l(e){var t=s[e];if(void 0!==t)return t.exports;var r=s[e]={id:e,exports:{}};return i[e](r,r.exports,l),r.exports}l.m=i,e="function"==typeof Symbol?Symbol("webpack queues"):"__webpack_queues__",t="function"==typeof Symbol?Symbol("webpack exports"):"__webpack_exports__",r="function"==typeof Symbol?Symbol("webpack error"):"__webpack_error__",n=e=>{e&&e.d<1&&(e.d=1,e.forEach((e=>e.r--)),e.forEach((e=>e.r--?e.r++:e())))},l.a=(o,a,i)=>{var s;i&&((s=[]).d=-1);var l,c,u,d=new Set,p=o.exports,h=new Promise(((e,t)=>{u=t,c=e}));h[t]=p,h[e]=e=>(s&&e(s),d.forEach(e),h.catch((e=>{}))),o.exports=h,a((o=>{var a;l=(o=>o.map((o=>{if(null!==o&&"object"==typeof o){if(o[e])return o;if(o.then){var a=[];a.d=0,o.then((e=>{i[t]=e,n(a)}),(e=>{i[r]=e,n(a)}));var i={};return i[e]=e=>e(a),i}}var s={};return s[e]=e=>{},s[t]=o,s})))(o);var i=()=>l.map((e=>{if(e[r])throw e[r];return e[t]})),c=new Promise((t=>{(a=()=>t(i)).r=0;var r=e=>e!==s&&!d.has(e)&&(d.add(e),e&&!e.d&&(a.r++,e.push(a)));l.map((t=>t[e](r)))}));return a.r?c:i()}),(e=>(e?u(h[r]=e):c(p),n(s)))),s&&s.d<0&&(s.d=0)},l.d=(e,t)=>{for(var r in t)l.o(t,r)&&!l.o(e,r)&&Object.defineProperty(e,r,{enumerable:!0,get:t[r]})},l.f={},l.e=e=>Promise.all(Object.keys(l.f).reduce(((t,r)=>(l.f[r](e,t),t)),[])),l.u=e=>e+".js",l.g=function(){if("object"==typeof globalThis)return globalThis;try{return this||new Function("return this")()}catch(e){if("object"==typeof window)return window}}(),l.o=(e,t)=>Object.prototype.hasOwnProperty.call(e,t),o={},a="aarch64-esr-web-www:",l.l=(e,t,r,n)=>{if(o[e])o[e].push(t);else{var i,s;if(void 0!==r)for(var c=document.getElementsByTagName("script"),u=0;u<c.length;u++){var d=c[u];if(d.getAttribute("src")==e||d.getAttribute("data-webpack")==a+r){i=d;break}}i||(s=!0,(i=document.createElement("script")).charset="utf-8",i.timeout=120,l.nc&&i.setAttribute("nonce",l.nc),i.setAttribute("data-webpack",a+r),i.src=e),o[e]=[t];var p=(t,r)=>{i.onerror=i.onload=null,clearTimeout(h);var n=o[e];if(delete o[e],i.parentNode&&i.parentNode.removeChild(i),n&&n.forEach((e=>e(r))),t)return t(r)},h=setTimeout(p.bind(null,void 0,{type:"timeout",target:i}),12e4);i.onerror=p.bind(null,i.onerror),i.onload=p.bind(null,i.onload),s&&document.head.appendChild(i)}},l.r=e=>{"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(e,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(e,"__esModule",{value:!0})},l.v=(e,t,r,n)=>{var o=fetch(l.p+""+r+".module.wasm"),a=()=>o.then((e=>e.arrayBuffer())).then((e=>WebAssembly.instantiate(e,n))).then((t=>Object.assign(e,t.instance.exports)));return o.then((t=>"function"==typeof WebAssembly.instantiateStreaming?WebAssembly.instantiateStreaming(t,n).then((t=>Object.assign(e,t.instance.exports)),(e=>{if("application/wasm"!==t.headers.get("Content-Type"))return console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n",e),a();throw e})):a()))},(()=>{var e;l.g.importScripts&&(e=l.g.location+"");var t=l.g.document;if(!e&&t&&(t.currentScript&&"SCRIPT"===t.currentScript.tagName.toUpperCase()&&(e=t.currentScript.src),!e)){var r=t.getElementsByTagName("script");if(r.length)for(var n=r.length-1;n>-1&&(!e||!/^http(s?):/.test(e));)e=r[n--].src}if(!e)throw new Error("Automatic publicPath is not supported in this browser");e=e.replace(/^blob:/,"").replace(/#.*$/,"").replace(/\?.*$/,"").replace(/\/[^\/]+$/,"/"),l.p=e})(),(()=>{var e={57:0};l.f.j=(t,r)=>{var n=l.o(e,t)?e[t]:void 0;if(0!==n)if(n)r.push(n[2]);else{var o=new Promise(((r,o)=>n=e[t]=[r,o]));r.push(n[2]=o);var a=l.p+l.u(t),i=new Error;l.l(a,(r=>{if(l.o(e,t)&&(0!==(n=e[t])&&(e[t]=void 0),n)){var o=r&&("load"===r.type?"missing":r.type),a=r&&r.target&&r.target.src;i.message="Loading chunk "+t+" failed.\n("+o+": "+a+")",i.name="ChunkLoadError",i.type=o,i.request=a,n[1](i)}}),"chunk-"+t,t)}};var t=(t,r)=>{var n,o,[a,i,s]=r,c=0;if(a.some((t=>0!==e[t]))){for(n in i)l.o(i,n)&&(l.m[n]=i[n]);s&&s(l)}for(t&&t(r);c<a.length;c++)o=a[c],l.o(e,o)&&e[o]&&e[o][0](),e[o]=0},r=self.webpackChunkaarch64_esr_web_www=self.webpackChunkaarch64_esr_web_www||[];r.forEach(t.bind(null,0)),r.push=t.bind(null,r.push.bind(r))})(),l.e(605).then(l.bind(l,605)).then((e=>{e.init();const t=document.getElementById("esr");null!=t&&(t.oninput=()=>{t.value.length>0&&e.decode_esr(t.value),window.location.hash=t.value},window.location.hash&&(t.value=window.location.hash.substring(1),e.decode_esr(t.value)));const r=document.getElementById("midr");null!=r&&(r.oninput=()=>{r.value.length>0&&e.decode_midr(r.value),window.location.hash=r.value},window.location.hash&&(r.value=window.location.hash.substring(1),e.decode_midr(r.value)));const n=document.getElementById("smccc");null!=n&&(n.oninput=()=>{n.value.length>0&&e.decode_smccc(n.value),window.location.hash=n.value},window.location.hash&&(n.value=window.location.hash.substring(1),e.decode_smccc(n.value)))}))})();
//# sourceMappingURL=index.js.map