# Redis WebAssembly Example

This project demonstrates how to use Redis with WebAssembly (Wasm) using Rust.

## Prerequisites

Before running the code, make sure you have the following installed:

- Rust: https://www.rust-lang.org/tools/install
- Redis: https://redis.io/topics/quickstart
- wasm-pack: https://rustwasm.github.io/wasm-pack/installer/

## Getting Started

Follow these steps to run the code:

1. Clone the repository:

   ```bash
   git clone https://github.com/your-username/redis-wasm.git
   
2. Navigate to the project directory:
   ```bash
   cd redis-wasm

3. Build the WebAssembly module using wasm-pack:
   ```bash
   wasm-pack build --target web
4. Start the Redis server. You can refer to the Redis documentation for instructions on starting the server based on your platform.

## Code Explanation

The Rust code in this project interacts with Redis using the redis crate. It provides two exported functions that can be called from JavaScript:

get_redis_key: Retrieves the value of a Redis key. It takes the Redis host, port, and key as input and returns the corresponding value.

increment_redis_key: Increments the value of a Redis key. It takes the Redis host, port, and key as input and increments the value by 1.
The Wasm module can be used in a web environment by including the generated JavaScript glue code (redis_wasm.js) and Wasm binary (redis_wasm_bg.wasm) in your HTML file.
