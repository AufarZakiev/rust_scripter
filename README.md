### Vue 3 & Rust combined

This demo unites:
- [Vue 3](https://vuejs.org/) (TS)
- [Egui](https://github.com/emilk/egui) (Rust)
- [Bundler](https://vitejs.dev/): Vite
- Component library for Vue: [Quasar](https://quasar.dev/)

#### Why is it cool

Rust part is compiled into WASM. WASM-based graphics (in "Demo" tab) performs in 60 FPS. It could be used to render
highly interactive components, e.g. editors, charts, etc.

Rust is also good at heavy computations. RustyAdder in "Demo" tab shows how data could be passed to Rust code and then the result goes back to JS.

Vue is commonly used JS framework. It isolates Rusty part of the app and makes it easy to create other app parts.
Vue's ecosystem (such as Quasar) could be used in full power.

#### Demo

Deployed to [Github pages](https://aufarzakiev.github.io/rust_scripter/)