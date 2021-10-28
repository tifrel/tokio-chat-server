This repo is a follow-along of
[an awesome tutorial on async Rust + Tokio](https://www.youtube.com/watch?v=Iapc-qGTEBQ)

## Commits/iterations on the server

1. A simple server that echoes what is being sent, and then exits.
2. Echo until the client disconnects, then exit.
3. Echo for multiple clients, never exit.
4. An actual chat server: Every sent message gets broadcastet to all connected
   clients.
5. When broadcasting, do not send the message back to the sender, as that
   duplicates the message on his terminal.

## Tokio concepts

- `Future<T>`: Rust equivalent of a JS `Promise<T>`
- `spawn`: Tasks similar goroutines in go, or coroutines, might be
  conceptualized as a lightweight thread
  - Passed block needs to be static and values are moved into it
- `select!` macro: Like a `match`, but for futures. Executes the first future
  that is resolved.
  - Specifically more appropriate than `spawn`, when multiple different
    tasks/blocks need to refer to the same piece of memory

## Possible follow-up

- implement your own `tokio::channel` function
