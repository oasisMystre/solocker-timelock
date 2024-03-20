## Solocker timelocker #Streamflow custom fork

\_Disclaimer: This is a Community (free and open-source) version of a [Streamflow protocol](https://github.com/streamflow-finance/js-sdk).

Functionalities are:

- `create` a vesting contract.
- `withdraw` from a vesting contract.
- `cancel` a vesting contract.
- `transfer_recipient` of a vesting contract.

#  Run Test Suite

To get started you must have openssl installed on your machine

```shell
brew install openssl
```

Install solana cli

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Supported rust version is 1.59
Switch your rust from $version to 1.58

```shell
rustup install 1.59
rustup default 1.59
```

Install solana cli using

```shell
sh -c "$(curl -sSfL https://release.solana.com/v1.9.13/install)"
```

If you have solana cli previously installed 
```shell
solana-install init 1.9.13
```

Then run the benchmark test using
```
cargo test-bpf
```