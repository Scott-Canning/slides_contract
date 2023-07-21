## Slides smart contract
### Build & deploy cycle commands

Recycle account: 
```
near delete slides.scxtt.testnet scxtt.testnet                                                 
near create-account slides.scxtt.testnet --masterAccount scxtt.testnet --initialBalance 5
```

Compile contract:
```
cargo build --all --target wasm32-unknown-unknown --release
```

Deploy contract:
```
near deploy slides.scxtt.testnet --wasmFile target/wasm32-unknown-unknown/release/slides.wasm
```

Check account state:
```
near state slides.scxtt.testnet
```

Check contract state:
```
near view-state slides.scxtt.testnet --finality final
```

## Unit tests
Test
```
cargo test
```
```
cargo test -- --nocapture
```
