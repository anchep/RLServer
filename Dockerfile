# 第一阶段：构建阶段
FROM rust:1.91.1 AS builder

# 安装PostgreSQL开发库
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 复制Cargo.toml和Cargo.lock文件
COPY Cargo.toml Cargo.lock ./

# 创建一个空的src目录和main.rs文件，用于构建依赖
RUN mkdir -p src && echo "fn main() {} " > src/main.rs

# 只构建主二进制文件的依赖（利用Docker缓存）
RUN cargo build --release --bin rlserver

# 删除临时的main.rs文件
RUN rm src/main.rs

# 复制所有源代码
COPY . .

# 构建所有应用二进制文件
RUN cargo build --release

# 第二阶段：运行阶段
FROM debian:bookworm-slim

# 安装PostgreSQL运行时库和curl（用于健康检查）
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# 设置工作目录
WORKDIR /app

# 从构建阶段复制构建好的二进制文件
COPY --from=builder /app/target/release/rlserver /app/rlserver

# 复制模板和静态资源
COPY templates/ /app/templates/
COPY static/ /app/static/

# 暴露应用端口
EXPOSE 28001

# 设置环境变量
ENV RUST_BACKTRACE=1
ENV TEMPLATES_DIR=/app/templates
ENV STATIC_DIR=/app/static

# 健康检查
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:28001/health || exit 1

# 运行应用，设置RUST_BACKTRACE=1以获取详细的错误信息
CMD ["sh", "-c", "RUST_BACKTRACE=1 ./rlserver"]
