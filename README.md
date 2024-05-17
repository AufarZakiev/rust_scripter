# Vue 3 + Rust (egui) combination project

This project is demo for uniting two different technologies:
- [Vue 3](https://vuejs.org/) (TS)
- [Egui](https://github.com/emilk/egui) (Rust)
- [Bundler](https://vitejs.dev/): Vite
- Component library for Vue: [Quasar](https://quasar.dev/)

Done to make a challenge and see how it works together. 
Egui part is incapsulated into "Editor" tab; Vue 3 wraps everything and provides common UX.

## Editor

Editor is a self-contained app used to create pipeline of [Rhai](https://rhai.rs/) scripts. 
It is very basic and misses a lot of features.

## Demo

Deployed to: https://aufarzakiev.github.io/rust_scripter/ .