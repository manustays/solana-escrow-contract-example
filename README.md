# Solana Smart Contract Example - Escrow Account
A simple escrow account example smart contract on Solana where two parties (say, Alice & Bob) can share their Tokens securely. The tokens are temporarily stored in the escrow account and the exchange happens only when both the parties have successfully transfered their tokens. The escrpw contract then send the exchanged tokens back to Alice & Bob.

### Program Flow
1. The entrypoint is called (see: [`entrypoint.rs`](src/entryopint.rs)).
2. entrypoint forwards args to processor (see: [processor.rs](src/processor.rs)).
3. processor asks [`instruction.rs`](src/instruction.rs) to decode the `instruction_data` argument from the entrypoint function.
4. Using the decoded data, the processor will now decide which processing function to use to process the request.
5. The processor may use [`state.rs`](src/state.rs) to encode state into or decode the state of an account which has been passed into the entrypoint.


### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build
```
$ cargo build
```

### Credits
https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
