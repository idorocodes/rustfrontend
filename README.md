
# Leptos Starter Template 

A minimal, opinionated **Leptos** starter for building modern, reactive web applications in **Rust**.
Designed for fast setup, clean structure, and easy extension.



## Tech Stack

* **Rust**
* **Leptos**
* **WASM**
* **Cargo**
* **Trunk** (frontend build)


## Getting Started

### Prerequisites

Make sure you have:

* Rust (latest stable)
* Cargo
* Trunk

  ```bash
  cargo install trunk
  ```
* WASM target

  ```bash
  rustup target add wasm32-unknown-unknown
  ```

### Install & Run (Development)

```bash
git clone https://github.com/idorocodes/leptosstarter.git
cd leptosstarter
trunk serve   --open
```

Open your browser at:
`http://localhost:8080`

## Build for Production

```bash
trunk build --release
```

Output will be in:

```
dist/
```

Deploy this directory to any static host (Netlify, Vercel, Cloudflare Pages, etc.).

## Customization

* Add routing with `leptos_router`
* Add server functions for full-stack apps
* Integrate Tailwind, Vanilla CSS, or CSS Modules
* Plug into Axum/Actix for SSR

 tired of JavaScript 

## License

MIT License.
Use it. Fork it. Ship fast.
