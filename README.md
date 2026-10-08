# DataCat-CLI

Command line utility for interacting with the DataCat API.

## Install

```
cargo install clicat
```

This will install the binary to `~/.cargo/bin`, so make sure to add this to your `PATH`.

## Usage

```
clicat --help
```

First one must configure and OAuth client, with

```
clicat auth-client add
```

Then, login with

```
clicat login
```
