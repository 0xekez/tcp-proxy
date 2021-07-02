# Proxy

A simple Rust TCP proxy.

## Terminoligy

`origin` - the listener we'd like to proxy tcp connections to.
`eyeball` - the initiator of connections to the origin.

## Benchmarking

Using localhost:

1. Start the proxy in one terminal window
   ```bash
   cargo run -- -e 0.0.0.0:1212 -o 127.0.0.1:1313
   ```
2. Listen on the origin port in another window
   ```
   nc -l 1313 > /dev/null
   ```
3. Send a bunch of traffic from another window
   ```
   dd if=/dev/zero bs=1024 count=900000 | nc -v 127.0.0.1 1212
   ```

## Local Results

Using the proxy:

```
~ dd if=/dev/zero bs=1024 count=900000 | nc -v 127.0.0.1 1212
Connection to 127.0.0.1 port 1212 [tcp/lupa] succeeded!
900000+0 records in
900000+0 records out
921600000 bytes transferred in 4.369209 secs (210930626 bytes/sec)
```

Connecting directly to the eyeball:

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
