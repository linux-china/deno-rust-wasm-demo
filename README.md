Deno Rust WebAssembly开发项目模板
===========================

这个项目是知道如何使用Rust为Deno开发WebAssembly，而且方便发布。 项目的模板来自 https://github.com/denosaurs/wasabi/generate

# 特性

* 一键编译:  deno run -A scripts/build.ts
* WebAssembly & JS打包: 将wasm文件和关联JS文件打包形成一个独立的wasm.js文件，方便发布
* 开发便捷性: 基于Rust test进行测试

# Deno and WebAssembly

* web-sys: fetch overload by fetch_with_*() - https://rustwasm.github.io/docs/wasm-bindgen/web-sys/function-overloads.html
* web-sys: WebSocket
* Crypto
* LocalStorage
* WebGPU: https://www.w3.org/TR/webgpu/
* Intl: 国际化 
* BroadcastChannel
* addEventListener() and dispatchEvent()
* Promises and Futures
* Reference Types

关于Windows对象的属性，请参考 https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Window.html 

# Prerequisites

- [deno](https://deno.land/)
- [rust](https://www.rust-lang.org/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)
- [just](https://github.com/casey/just)

# IDE support

建议使用Clion打开该项目，同时安装Deno plugin。
