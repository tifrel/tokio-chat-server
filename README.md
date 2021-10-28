This repo is a follow-along of
[an awesome tutorial on async Rust + Tokio](https://www.youtube.com/watch?v=Iapc-qGTEBQ)

## Commits/iterations on the server

1. A simple server that echoes what is being sent, and then exits.
2. Echo until the client disconnects, then exit.
3. Echo for multiple clients, never exit.
4. An actual chat server: Every sent message gets broadcastet to all connected
   clients.

## Tokio concepts

- `Future<T>`: Rust equivalent of a JS `Promise<T>`
- Tasks: Lightweight threads (like goroutines in go, or coroutines)
- `select!` macro: Like a `match`, but for futures. Executes the first future
  that is resolved.
