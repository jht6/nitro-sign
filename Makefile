# Nitro Enclaves 构建辅助。
#
# 首次使用需安装 musl 目标（生成 enclave 所需的静态二进制）：
#   rustup target add $(uname -m)-unknown-linux-musl

ARCH := $(shell uname -m)
EIF := enclave.eif

.PHONY: all host enclave build-enclave eif run-enclave clean

all: host enclave

# host 跑在父实例，用默认 gnu 目标即可
host:
	cargo build --release --bin host

# enclave 需要 musl 静态二进制，打进 EIF
enclave:
	cargo build --release --bin enclave --target $(ARCH)-unknown-linux-musl
	cp target/$(ARCH)-unknown-linux-musl/release/enclave ./enclave

# 构建 enclave docker 镜像
build-enclave: enclave
	docker build -t nitro-sign-enclave -f Dockerfile.enclave .

# 构建 EIF
eif: build-enclave
	nitro-cli build-enclave --docker-uri nitro-sign-enclave --output-file $(EIF)

# 运行 enclave（2 vCPU / 256 MiB，可按需调整）
run-enclave: eif
	nitro-cli run-enclave --eif-path $(EIF) --cpu-count 2 --memory 256

clean:
	cargo clean
	rm -f enclave $(EIF)
