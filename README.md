# Kiwi Key-Value Store Server

Kiwi is a lightweight key-value store server implemented in Rust, using asynchronous programming models (via `tokio`) and supporting a simple web interface via `actix-web`.

## Features

- Basic key-value operations: `GET`, `SET`, and `DEL`.
- Expire mechanism for keys (`TTL`).
- HTTP-based API interface.
- Compatibility note for `redis-cli` using a custom command handler.

## API Endpoints

- `POST /set`: Sets a value for a given key. Allows specifying a TTL.

  - Request body: `{"key": "some_key", "value": "some_value", "ttl": 3600}`
  - Response: JSON representation of the stored content.

- `POST /get`: Retrieves a value for a given key.

  - Request body: `{"key": "some_key"}`
  - Response: JSON representation of the content or `404` if not found/expired.

- `POST /del`: Deletes a given key.
  - Request body: `{"key": "some_key"}`
  - Response: `"OK"` if successful or an error message.

## Using with `redis-cli`

Note: You will need a local proxy or adaptation layer that translates `redis-cli` commands into the corresponding HTTP requests. This is not provided out-of-the-box.

So refactor main.rs instead of using http to use cmds

## How to Run

1. Ensure you have Rust installed.
2. Clone the repository and navigate into the project directory.
3. Build the project using Cargo:

```sh
cargo run
```

5. Server will start by default at `http://localhost:8080`.

## Commands Reference

Here are equivalent commands for our server translated from typical Redis usage:

- To set a key:

```sh
curl -X POST "http://localhost:8080/set " -H "Content-Type: application/json" -d '{"key": "name", "value": "kiwi", "ttl": 3600}'
```

- To get a key:

```sh
curl -X POST "http://localhost:8080/get " -H "Content-Type: application/json" -d '{"key": "name"}'
```

- To delete a key:

```sh
curl -X POST "http://localhost:8080/del " -H "Content-Type: application/json" -d '{"key": "name"}'
```
