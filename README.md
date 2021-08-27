# Turbocharger

## > WORK IN PROGRESS <

Autogenerated async RPC bindings that instantly connect a JS frontend to a Rust backend service via WebSockets and WASM.

Makes a Rust _backend_ function, e.g.:

```rust
#[turbocharger::backend]
async fn get_person(id: i64) -> Person {
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

Full backend and frontend project in `example-turbosql/`; run with `cargo run example-turbosql`

### `main.rs`

```rust
use serde::{Serialize, Deserialize};
use turbocharger::{backend, server_only};
use turbosql::{Turbosql, select};

#[backend]
#[derive(Turbosql, Serialize, Deserialize, Default)]
struct Person {
 rowid: Option<i64>,
 name: Option<String>
}

#[backend]
async fn insert_person(p: Person) -> anyhow::Result<i64> {
 p.insert() // returns Result<rowid>
}

#[backend]
async fn get_person(rowid: i64) -> anyhow::Result<Person> {
 turbosql::select!(Person "WHERE rowid = ?", rowid)
}

#[server_only]
#[tokio::main]
async fn main() {
 eprintln!("Serving on http://127.0.0.1:8080");
 warp::serve(turbocharger::warp_routes()).run(([127, 0, 0, 1], 8080)).await;
}
```

### `index.js`

```js
import turbocharger_init, { backend } from "./turbocharger_generated";

(async () => {
 await turbocharger_init();
 let rowid = await backend.insert_person({ name: "Bob" });
 console.log(await backend.get_person(rowid));
})();
```

## Usage

Your `main.rs` file is the entry point for both the server `bin` target and a `wasm-bindgen` `lib` target. The `#[backend]` macro outputs three functions:

- Your function, unchanged, for the server `bin` target; you can call it directly from other server code if you wish.
- An internal function for the server `bin` target providing the RPC glue.
- A `#[wasm_bindgen]` function for the frontend WASM module that makes the RPC call and delivers the response.

All functions and structs in `main.rs` should be annotated with one of `#[backend]`, `#[server_only]`, or `#[wasm_only]`.

## Server

Currently, the server side is batteries-included with `warp`, but this could be decoupled in the future. If this would be useful for you, please open a GitHub issue describing a use case.

## WASM functions

You can also easily add standard `#[wasm-bindgen]` style Rust functions to the WASM module, accessible from the frontend only:

```rust
use turbocharger::wasm_only;

#[wasm_only]
async fn get_wasm_greeting() -> String {
 "Hello from WASM".to_string()
}
```

## To Do / Future Directions

- Better WebSocket status management / reconnect
- Error handling with `Result::Err` triggering a Promise rejection
- Streaming responses with `futures::stream`
- `Vec<T>` types, see [wasm-bindgen#111](https://github.com/rustwasm/wasm-bindgen/issues/111)
- Anything [`tarpc`](https://github.com/google/tarpc) does, particularly around timeouts, cancellation, etc.

### License: MIT OR Apache-2.0 OR CC0-1.0 (public domain)
