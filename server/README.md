# 文档管理系统 - 后端

基于 Rust + Axum 构建的高性能文档管理系统后端。

## 技术栈

- **语言**: Rust 1.70+
- **Web 框架**: Axum 0.7
- **ORM**: Diesel 2.1
- **数据库**: PostgreSQL 15+
- **对象存储**: MinIO (S3 兼容)
- **搜索引擎**: MeiliSearch
- **文档编辑**: OnlyOffice
- **认证**: JWT

## 项目结构

```
server/
├── src/
│   ├── main.rs             # 应用入口
│   ├── config.rs           # 配置管理
│   ├── db.rs              # 数据库连接
│   ├── error.rs           # 错误处理
│   ├── schema.rs          # Diesel 模式
│   ├── handlers/          # 请求处理器
│   ├── middleware/        # 中间件
│   ├── models/            # 数据模型
│   ├── routes/            # 路由定义
│   ├── services/          # 业务服务
│   └── utils/             # 工具函数
├── migrations/            # 数据库迁移
├── Cargo.toml            # 依赖配置
└── diesel.toml           # Diesel 配置
```

## 快速开始

### 前置要求

- Rust 1.70+
- PostgreSQL 15+
- Docker 和 Docker Compose（用于依赖服务）

### 安装依赖

```bash
cd server
```

### 配置环境

创建 `.env` 文件（参考根目录的 `.env`）。

### 启动依赖服务

```bash
cd ..
docker-compose up -d
```

### 运行应用

```bash
cargo run
```

应用将在 `http://localhost:8080` 启动。

## 开发

### 运行测试

```bash
cargo test
```

### 代码格式化

```bash
cargo fmt
```

### 代码检查

```bash
cargo clippy
```

### 构建发布版本

```bash
cargo build --release
```

## 数据库迁移

应用启动时会自动运行迁移。你也可以使用 Diesel CLI（如果安装了）：

```bash
# 安装 Diesel CLI (可选)
cargo install diesel_cli --no-default-features --features postgres

# 运行迁移
diesel migration run

# 回滚迁移
diesel migration revert

# 查看迁移状态
diesel migration list
```

## API 端点

详见根目录的 [API.md](../API.md) 文档。

## 许可证

MIT License

