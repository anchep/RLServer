# 使用Rust 1.86版本作为构建环境
FROM rust:1.86 as builder

# 设置工作目录
WORKDIR /app

# 复制所有源代码和配置文件
COPY . .

# 构建应用，跳过dev-dependencies
RUN cargo build --release

# 使用最新版的Debian镜像作为运行环境
FROM debian:latest

# 安装PostgreSQL客户端（如果需要）
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建环境复制编译好的应用
COPY --from=builder /app/target/release/rlserver ./

# 暴露应用端口
EXPOSE 28001

# 设置环境变量
ENV RUST_BACKTRACE=1

# 运行应用
CMD ["./rlserver"]
