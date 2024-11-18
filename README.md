# faucet <img src="docs/figures/faucet.png" align="right" width=120 height=139 alt="" />

<!-- badges: start -->
[![Crates.io](https://img.shields.io/crates/v/faucet-server.svg)](https://crates.io/crates/faucet-server)
[![test](https://github.com/ixpantia/faucet/actions/workflows/test.yaml/badge.svg?branch=main)](https://github.com/ixpantia/faucet/actions/workflows/test.yaml)
<!-- badges: end -->

The Ultimate solution for running and deploying Data Applications written in R.

- 🚀&nbsp; [Features](#id-features)
- ⬇️&nbsp; [Installing faucet](#id-install)
- 🐋&nbsp; [Docker](#id-docker)



## 🚀&nbsp; Features <a id="id-features">

- Works on macOS, Linux and Windows.
- Scale Shiny Applications and Plumber APIs
- Supports advanced logging and telemetry.
- Simplifies deployment of R applications in Docker.
- Supports x86-64 and ARM64 systems.
- Supports routing to Shiny applications and Plumber APIs.

## ⬇️&nbsp; Installing faucet <a id="id-install">

- [Linux](#id-install-linux)
- [cargo (macOS, Linux, Windows)](#id-install-cargo)

### Installing faucet on Linux <a id="id-install-linux">

On Linux you can install faucet by downloading the binary directly. The binary
will depend on the architecture of your machine and the version of faucet you
wish to download.

You can install faucet with the following script:

```bash
FAUCET_VERSION="v1.1.0"
ARCH="x86_64" # This could also be "aarch64" is running ARM64

wget https://github.com/ixpantia/faucet/releases/download/$FAUCET_VERSION/faucet-$ARCH-unknown-linux-musl -O faucet

# Make the binary executable
chmod +x faucet

# Install the binary to a directory in your PATH (e.x., usr local bin)
sudo install faucet /usr/local/bin/faucet
```

### Installing faucet on using `cargo` <a id="id-install-cargo">

To install `faucet` using `cargo` you can run:

```sh
cargo install faucet-server
```

## 🐋&nbsp; faucet in Docker <a id="id-docker"> 

faucet is the easiest way to run Shiny Applications and Plumber APIs in Docker.
You can use the base image `ixpantia/faucet` to get automatic access to R
and the faucet runtime.

A minimal `Dockerfile` using faucet can look like this:

```dockerfile
FROM ixpantia/faucet:latest

RUN Rscript -e "install.packages('shiny')"

COPY app.R .
```

You can also specify and R version and/or a faucet version. For example, if
we wanted to run this using R version 4.3 we can change the tag to
`ixpantia/faucet:r4.3`.

```dockerfile
FROM ixpantia/faucet:r4.3

RUN Rscript -e "install.packages('shiny')"

COPY app.R .
```
