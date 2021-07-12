# Proxy

A simple Rust TCP proxy. Intended to be educational - [don't use this](https://github.com/tokio-rs/tokio/discussions/3173) in production.

## Terminoligy

`server` - the listener we'd like to proxy tcp connections to.
`client` - the initiator of connections to the server.

## Benchmarking

### Using localhost:

1. Start the proxy in one terminal window
   ```bash
   cargo run -- -c 0.0.0.0:1212 -s 127.0.0.1:1313
   ```
2. Listen on the origin port in another window
   ```
   nc -l 1313 > /dev/null
   ```
3. Send a bunch of traffic from another window
   ```
   dd if=/dev/zero bs=1024 count=900000 | nc -v 127.0.0.1 1212
   ```
### Using network namespaces:

If you have a debian linux machine you can use `ns_test.sh` to run a
benchmark using three network namespaces.

## Local Results

Using the proxy:

```
~ dd if=/dev/zero bs=1024 count=900000 | nc -v 127.0.0.1 1212
Connection to 127.0.0.1 port 1212 [tcp/lupa] succeeded!
900000+0 records in
900000+0 records out
921600000 bytes transferred in 4.369209 secs (210930626 bytes/sec)
```

Connecting directly to the server:

```
~ dd if=/dev/zero bs=1024 count=900000 | nc -v 127.0.0.1 1313
Connection to 127.0.0.1 port 1313 [tcp/bmc_patroldb] succeeded!
900000+0 records in
900000+0 records out
921600000 bytes transferred in 3.612901 secs (255085873 bytes/sec)
```

Results

```elisp
(* 100 (/ 210930626.0 255085873.0))
=> 82.69004610851186
```

The tcp proxy is 82% the speed of a regular connection.

## Network Namespace Results

Using the proxy:

```
921600000 bytes (922 MB, 879 MiB) copied, 25.4762 s, 36.2 MB/s
```

Connecting directly to the server:

```
921600000 bytes (922 MB, 879 MiB) copied, 5.17278 s, 178 MB/s
```

Results:

```elisp
(* 100 (/ 36.2 178))
=> 20.337078651685395
```

the tcp proxy is 20% the speed of a regular connection.
