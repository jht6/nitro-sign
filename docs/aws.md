1. 执行 `make`
2. 执行 `make run-enclave`
3. 另一终端，执行 `NITRO_ENCLAVE_CID=16 cargo run --bin host`
4. 请求测试: `curl http://127.0.0.1:8080/demo`