# yamldump

Dump and inspect YAML files from the terminal.

## Install

```console
cargo build --release
sudo cp target/release/yamldump /usr/local/bin/
```

## Usage

```console
yamldump config.yaml
yamldump deployment.yml | grep image
```

Output:

```yaml
image: nginx:latest
replicas: 3
```
