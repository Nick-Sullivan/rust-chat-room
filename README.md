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
```

Or via VSCode (Run -> API Server).

In two terminals, run:

```bash
wscat -c ws://localhost:3000
UserUpdate:RoomId=room1&Name=name1 # can use distinct names
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

In two terminals, run:

```bash
export WSS=<value of api_gateway_url>
wscat -c $WSS
UserUpdate:RoomId=room1&Name=name1 # can use distinct names
```

Send a message in one and see it reflected in the others.

# Running locally, but using the cloud database

In `launch.json`, uncomment the `WEBSOCKET_TABLE_NAME` variable.

Or via VSCode (Run -> API Server).

In two terminals, run:

```bash
wscat -c ws://localhost:3000
RoomId:room1
UserUpdate:RoomId=room1&Name=name1 # can use distinct names
```
