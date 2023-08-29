# lwa_simple_server

[![build](https://github.com/Leinnan/lwa_simple_server/actions/workflows/rust.yml/badge.svg)](https://github.com/Leinnan/lwa_simple_server/actions/workflows/rust.yml)

Simple server made with hosting locally webgl games in mind.

It makes testing Unity webgl games easy, even allows connecting with different domains(less CORS issues during tests).

> **ACHTUNG!**
>
> **WARNING!**
>
> It is **NOT** meant to use in production environment

## Installation

For now it is required to have cargo and rust installed:

```bash
git clone https://github.com/Leinnan/lwa_simple_server.git
cd lwa_simple_server
cargo install --path .
```

## Usage

To start run it in folder that should be root folder of hosted site:

```bash
cd desired/folder
lwa_simple_server "folder_to_host"
```


## SSL

If you would like to use OpenSSL create key with command below and pass paths to generated files as arguments in command in order to use it:

```bash
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```
