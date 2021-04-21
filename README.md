# lwa_simple_server

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
lwa_simple_server
```

After first start of the program in configs folder will be created `lwa_simple_server` directory with `lwa_simple_server.toml` in it with config values:

```toml
folder_to_host = '/example/path/TestProject'
bind_address = 'localhost:8000'
private_key_file = 'key.pem'
certificate_chain_file = 'cert.pem'
use_ssl = false
```

## SSL

If you would like to use OpenSSL create key with command below and pass paths to generated files in config above in order to use it:

```bash
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'
```
