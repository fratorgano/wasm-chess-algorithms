# wasm-chess-algorithms
Node.js packet written in rust to provide a more optimized chess game tree exploration than the one possible using Javascript.
This uses WASM to convert rust code to create a node.js package that can be used from the server-side of the node.js application.

I developed a website to play chess online that uses this package and it's available [here](https://github.com/fratorgano/SocketChess).

## Implemented algorithms
* Random - Chooses a random move between options
* Negamax - Simpler to implement version of Minimax
* Negamax AB - Negamax optimized with alpha-beta pruning
* Negamax AB Table - Negamax AB supported by a simple transposition table
* Negamax AB Quiescent - Negamax AB with quiescent search at the end of normal search
* Iterative Deepening - Negamax AB repeated with increased depth each time until there's time
* Iterative Deepening Table - Iterative Deepening that uses a transposition table to store results between iterations
* Iterative Deepening Order - Iterative Deepening that orders move before iterating to improve performance

## Commands
`wasm-pack build --target nodejs --out-dir path-to-node-modules-folder` - Compile rust code and create node.js module based on it

`cargo test` - Compile rust code and test it

## Dependencies/Modules Used
* [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Helping with Rust-Node.js integration
* [Serde](https://github.com/serde-rs/serde) - Framework used to serialize and deserialize data to be able to transfer between Rust and Node.js
* [shakmaty](https://github.com/niklasf/shakmaty) - Library for chess move generation
* [rand](https://github.com/rust-random/rand) - Library for seeded random number generation
* [web-sys](https://github.com/rustwasm/wasm-bindgen/tree/main/crates/web-sys) - Using Web APIs with wasm-bindgen
* [instant](https://crates.io/crates/instant) - Replacement for std::time::Instant on WASM
