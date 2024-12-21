## Rust Chat Room

A basic websocket server written in rust.

## Build

```bash
cd server
cargo build
```

## Running locally as a server

Via command line.

```bash
cd server
cargo run
wscat -c ws://localhost:3000
```

## Creating cloud infrastructure

To create this infrastructure, we use terraform.

```
cd terraform
terraform init
terraform apply
```

In the output, take note of the `api_gateway_url`.

To clean up at the end:

```bash
terraform destroy
```

# Chatting to the cloud

```bash
export WSS=<value of api_gateway_url>
wscat -c $WSS
```
