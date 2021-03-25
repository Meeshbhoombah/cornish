# server

## Getting Started
### Prerequisites
Download Rust

### Installation
### Testing
Run all tests at once with `cargo test`.
```
$ cargo test
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/socket-641aa82b1a3c8a8e

running 1 test
test _server_pong_on_ping ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

It can be more convient to run a single test, which you can do by including the
file name of the paritcular test you wish to run, without the file extension, or
`cargo test socket`.
```
$ cargo test socket
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running target/debug/deps/cornish_server-157dc68d1cb5b73c

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running target/debug/deps/socket-641aa82b1a3c8a8e

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s
```

