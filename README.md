# example-subxt

About Usage example subxt

## Install polkadot binary and run

```
git clone https://github.com/paritytech/polkadot.git
cd ./polkadot
cargo build --release
./target/release/polkadot --dev --tmp

```

## install subxt-cli to you locak path 
```
git clone https://github.com/paritytech/subxt
cd ./subxt/cli
cargo install --path .
```


## Run example-subxt



This example source code from subxt(https://github.com/paritytech/subxt)

```
cd ./example-subxt/metadata/
subxt metadata -f bytes > metadata.scale

cd ./example-subxt

cargo run 
```

## Have some bug
```
example-subxt  🍣 main 🦀 v1.58.0-nightly 🐏 6GiB/8GiB | 5GiB/6GiB
🕙 15:54:15 ✖  cargo run
warning: unused import: `EventSubscription`
 --> src/main.rs:4:5
  |
4 |     EventSubscription,
  |     ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `example-subxt` (bin "example-subxt") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.78s
     Running `target/debug/example-subxt`
Error: Rpc(Request("{\"jsonrpc\":\"2.0\",\"error\":{\"code\":1010,\"message\":\"Invalid Transaction\",\"data\":\"Transaction has a bad signature\"},\"id\":5}"))

```

This bug can be fix, the author subxt have give answer.
This relate issue: https://github.com/paritytech/subxt/issues/338

update subxt git rep to : subxt = { version = "0.15.0", git = "https://github.com/paritytech/subxt.git", rev = "a701d80e24fdc" }

