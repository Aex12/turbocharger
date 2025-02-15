# Turbocharger

[<img alt="github" src="https://img.shields.io/badge/github-trevyn/turbocharger-663399?style=for-the-badge&labelColor=555555&logo=github" height="27">](https://github.com/trevyn/turbocharger)
[<img alt="crates.io" src="https://img.shields.io/crates/v/turbocharger.svg?style=for-the-badge&color=ffc833&logo=rust" height="27">](https://crates.io/crates/turbocharger)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-turbocharger-353535?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K" height="27">](https://docs.rs/turbocharger)

Autogenerated async RPC bindings that instantly connect a JS frontend to a Rust backend service via WebSockets and WASM.

See [https://github.com/trevyn/turbocharger-template](https://github.com/trevyn/turbocharger-template) for a full turnkey template repository.

Makes a Rust _backend_ function, e.g.:

```rust
#[turbocharger::backend]
pub async fn get_person(id: i64) -> Person {
 // ... write any async backend code here; ...
 // ... query a remote database, API, etc. ...
 Person { name: "Bob", age: 21 }
}
```

instantly available, with _no additional boilerplate_, to a frontend as

- an async JavaScript function
- with full TypeScript type definitions
- that calls the backend over the network:

```js
// export function get_person(id: number): Promise<Person>;

let person = await backend.get_person(1);
```

Works with any types that are [supported by](https://rustwasm.github.io/docs/wasm-bindgen/reference/types.html) `wasm-bindgen`, which includes most basic types and custom `struct`s with fields of supported types, but [not yet](https://github.com/rustwasm/wasm-bindgen/pull/2631) `enum` variants with values (which would come out the other end as TypeScript discriminated unions).

## How It Works

A proc macro auto-generates a frontend `wasm-bindgen` module, which serializes the JS function call parameters with `bincode`. These requests are sent over a shared WebSocket connection to a provided `warp` endpoint on the backend server, which calls your Rust function and serializes the response. This is sent back over the WebSocket and resolves the Promise returned by the original function call.

Multiple async requests can be simultaneously in-flight over a single multiplexed connection; it all just works.

## Complete Example: A full SQLite-powered backend with frontend bindings

See [https://github.com/trevyn/turbocharger-template](https://github.com/trevyn/turbocharger-template) for a full turnkey template repository.

### `backend.rs`

```rust
use turbocharger::backend;

#[backend]
#[derive(turbosql::Turbosql)]
pub struct Person {
 pub rowid: Option<i64>,
 pub name: Option<String>,
}

#[backend]
pub async fn insert_person(p: Person) -> Result<i64, turbosql::Error> {
 p.insert() // returns rowid
}

#[backend]
pub async fn get_person(rowid: i64) -> Result<Person, turbosql::Error> {
 turbosql::select!(Person "WHERE rowid = ?", rowid)
}
```

### `server.rs`

```rust
mod backend;

#[tokio::main]
async fn main() {
 #[derive(rust_embed::RustEmbed)]
 #[folder = "build"]
 struct Frontend;

 eprintln!("Serving on http://127.0.0.1:8080");
 warp::serve(turbocharger::warp_routes(Frontend)).run(([127, 0, 0, 1], 8080)).await;
}

```

### `index.js`

```js
import turbocharger_init, * as backend from "./turbocharger_generated";

(async () => {
 await turbocharger_init();
 let person = Object.assign(new backend.Person(), { name: "Bob" });
 let rowid = await backend.insert_person(person);
 console.log((await backend.get_person(rowid)).toJSON());
})();
```

## Usage

Start a new project using [https://github.com/trevyn/turbocharger-template](https://github.com/trevyn/turbocharger-template) for the full project layout and build scripts.

Your `backend.rs` module is included in both the server-side `bin` target in `server.rs` and a `wasm-bindgen` `lib` target in `wasm.rs`. The `#[backend]` macro outputs three functions:

- Your function, unchanged, for the server `bin` target; you can call it directly from other server code if you wish.
- An internal function for the server `bin` target providing the RPC dispatch glue.
- A `#[wasm_bindgen]` function for the frontend `lib` target that makes the RPC call and delivers the response.

Note that `backend.rs` is compiled to both `wasm32-unknown-unknown` and the host triple, and that you can annotate functions and structs in `backend.rs` with one of `#[backend]`, `#[server_only]`, or `#[wasm_only]`.

## Error Handling

`#[backend]` functions that need to return an error can return a `Result<T, E>` where `T` is a `wasm-bindgen`-compatible type and `E` is a type that implements `std::error::Error`, including `Box<dyn std::error::Error>>` and `anyhow::Error`. Errors crossing the network boundary are converted to a `String` representation on the server via their `to_string()` method and delivered as a Promise rejection on the JS side.

## Server

Currently, the server side is batteries-included with `warp`, but this could be decoupled in the future. If this decoupling would be useful to you, please open a GitHub issue describing a use case.

## WASM-only functions

You can also easily add standard `#[wasm-bindgen]`-style Rust functions to `wasm.rs`, accessible from the frontend only:

```rust
#[wasm-bindgen]
pub async fn get_wasm_greeting() -> String {
 "Hello from WASM".to_string()
}
```

### License: MIT OR Apache-2.0 OR CC0-1.0 (public domain)
