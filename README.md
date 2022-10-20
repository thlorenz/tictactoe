# TicTacToe

A [TicTacToe](https://en.wikipedia.org/wiki/Tic-tac-toe) game that runs on the
[Solana](https://solana.com/) blockchain.

It is built using the _MetaStack_ consisting of the following tools:

- [shank](https://github.com/metaplex-foundation/shank) which extracts IDL data from the Rust
  program
- [solita](https://github.com/metaplex-foundation/solita) which generates a TypeScript SDK from
  the extracted IDL
- [amman](https://github.com/metaplex-foundation/amman) which allows debugging and testing the
  program on a local validator

## Workshop

This workshop is implemented via pull requests. Each pull request adds a tool or feature and
explains how this was done and how you as a developer can experiment with the contract at the
current stage.

In most cases this involves building the Rust program, updating the SDK and running some
functions against that program after it is deployed on a local validator via _amman_.

### Prerequisites

- [install solana](https://docs.solana.com/cli/install-solana-cli-tools) in order to make a
  local validator available on your machine as well as gain access to commands like 
  `cargo build-bpf`

### Building the Rust Program

```sh
cd program && cargo build-bpf
```

### Installing SDK Dependencies

```sh
cd ts && yarn install 
```

### Generating SDK

```sh
cd ts && yarn api:gen
```

### Running SDK Tests

```sh
cd ts && yarn test
```

## Steps

**NOTE**: 

Each step/pull-request is tagged and thus you can check them out via:

`git switch <step> -c pr/<step>`.

For example to checkout step **2** you'd do: `git switch 2 -c pr/2`.

1. [adding solana rust program with processor method stubs](https://github.com/thlorenz/tictactoe/pull/1)
2. [setup SDK package](https://github.com/thlorenz/tictactoe/pull/2)
3. [preparing SDK generation](https://github.com/thlorenz/tictactoe/pull/3)
4. [generating IDL and TypeScript SDK](https://github.com/thlorenz/tictactoe/pull/4)
5. [setup amman and add initialize game test](https://github.com/thlorenz/tictactoe/pull/5)

## Resources

- [Figma Board](https://www.figma.com/file/mjzOnSoQ6cOlmRhCLFYgxn/Shank%2FSolita%2FAmman-Solana-Development-Toolbelt)
  used to explain the _MetaStack_.

## LICENSE

MIT
