# berry-battle-server
Berry Battle Server is a server side game simulator that officiates a match between two user defined AI algorithms executing on Raspberry Pi devices

## Build and Run
The application is configured to be executed on the same machine as the berry-battler and queries localhost for the gRPC server.

To configure the application to run on a different machine, the ```main.rs``` file should be updated with the correct IP address of the gRPC server (berry-battler) in place of "[::1]".

For the most accurate benchmarking results the binary should be build with the ```release``` flag. The code that performs benchmarking is wrapped in a ```test``` module. To execute and see the output of the benchmark test add the ```-- --nocapture``` flag (note the extra "--" is required).
```console
$ cargo build --release
$ cargo test --release -- --nocapture
```

