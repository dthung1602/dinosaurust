<div align="center">
    <img src="dinosaurust.png" style="width: 50%">
    <p>A DNS server written in Rust</p>
</div>

[//]: # (image from https://www.vecteezy.com/vector-art/4959393-nice-orange-dinosaur)

## Run locally

```shell
# Run DNS server on port 2053
cargo run --bin dinosaurust

# Run test client
cargo run --bin test_client

# Test DNS server with dig
dig @127.0.0.1 -p 2053 +nodnssec +noedns www.google.com

```

## Available Options

```shell
Usage: dinosaurust [OPTIONS]

Options:
      --ip <IP>                     [default: 0.0.0.0]
      --port <PORT>                 [default: 2053]
      --forward-server-ip <IP>      [default: 8.8.8.8]
      --forward-server-port <PORT>  [default: 53]
  -h, --help                        Print help
  -V, --version                     Print version


```

## Roadmap

- [x] Set up UDP server
- [x] Send ping pong
- [x] Write header
- [x] Write answer section
- [x] Parse header
- [x] Parse question section
- [x] Compress
  - [x] Parsing
  - [x] Writing
- [x] Forward to other server
- [ ] Caching policy
- [x] Other record types: AAAA, CNAME, NS, etc
- [ ] Maintain own database