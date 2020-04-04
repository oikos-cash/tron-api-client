# tron-api-client

[![crates.io](https://meritbadge.herokuapp.com/tron_api_client)](https://crates.io/crates/tron-api-client)
[![documentation](https://docs.rs/tron-api-client/badge.svg)](https://docs.rs/tron-api-client)
[![Build Status](https://travis-ci.org/kevholder/tron-api-client.svg?branch=master)](https://travis-ci.org/kevholder/tron-api-client)
[![Build Status Appveyor](https://ci.appveyor.com/api/projects/status/github/kevholder/tron-api-client)](https://ci.appveyor.com/project/kevholder/tron-api-client)

_WORK IN PROGRESS (alpha)_

A Rust library for Tron's HTTP APIs.

## Install

See [releases](https://github.com/kevholder/tron-api-client/releases).

If you have Rust: `cargo install tron_api_client`.

## Usage

```bash
$ tron help
tron 0.0.1
Kevin Holder <kevholderx@gmail.com>
Tron API client and CLI

USAGE:
    tron [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --network <network>    Specify tron network (uses trongrid.io) [env: TRON_NETWORK=]  [default: main]  [possible
                               values: main, shasta]

SUBCOMMANDS:
    get_account                   Get Account
    get_block_by_id               Get Block by Id
    get_block_by_num              Get Block by Number
    get_chain_parameters          Get Chain Parameters
    get_contract                  Get Contract
    get_node_info                 Get Node Info
    get_now_block                 Get Latest Block
    get_transaction_by_id         Get Transaction by Id
    get_transaction_info_by_id    Like get_transaction_by_id but more detailed
    help                          Prints this message or the help of the given subcommand(s)
    list_nodes                    List Nodes
```
