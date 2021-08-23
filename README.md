# Solana Example - Escrow Account
A simple escrow account example Solana smart contract (program) where two parties (say, Alice & Bob) can share their Tokens securely. The tokens are temporarily stored in the escrow account and the exchange happens only when both the parties have successfully transfered their tokens. The escrpw contract then send the exchanged tokens back to Alice & Bob.

### Program Flow
1. The entrypoint is called (see: [`entrypoint.rs`](src/entryopint.rs)).
2. entrypoint forwards args to processor (see: [processor.rs](src/processor.rs)).
3. processor asks [`instruction.rs`](src/instruction.rs) to decode the `instruction_data` argument from the entrypoint function.
4. Using the decoded data, the processor will now decide which processing function to use to process the request.
5. The processor may use [`state.rs`](src/state.rs) to encode state into or decode the state of an account which has been passed into the entrypoint.


### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build & Deploy on localnet

1. Run `cargo build-bpf` command to compile the program to a file with the so file extension.
2. Run `solana-keygen new` to create and save a solana keypair locally. (Or, [create a cli wallet](https://docs.solana.com/wallet-guide/cli) of your choosing.)
3. Start the localnet with the command `solana-test-validator`
   1. When calling `solana config get`, your "RPC URL" should now equal http://localhost:8899
   2. If not, run `solana config set --url http://localhost:8899`
   3. Running `solana balance` will show your balance which should NOT be 0. If it is, stop the validators, make sure you have created a key with `solana-keygen new` and start it again from genesis with `solana-test-validator -r`
4. Run `solana deploy PATH_TO_PROGRAM` command to deploy the program to localnet.
   1. The PATH_TO_PROGRAM will have been printed by `cargo build-bpf` command from previous step.
   2. It will print the program id which can be used in the UI to interact with the web3 API.



### Credits
https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
