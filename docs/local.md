0. Linux环境下，确保已有vsock
```
ls -la /dev/vsock
```

1. 启动enclave服务
```
cargo run --bin enclave
```

2. 启动host服务
```
NITRO_ENCLAVE_CID=1 cargo run --bin host
```

3. 请求
```
curl --noproxy '*' http://127.0.0.1:8080/demo
```