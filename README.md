# 文档管理系统

一个功能完善的文档管理系统，支持文件存储、在线编辑、权限管理、分享等功能。

## ✨ 功能特性

- 📁 **文件管理**：文件和文件夹的创建、上传、下载、删除、移动
- 📝 **在线编辑**：集成 OnlyOffice Document Server 支持 Word、Excel、PowerPoint 在线编辑
- 🔐 **权限管理**：细粒度的文档权限控制（读、写、删除、分享、管理）
- 🔗 **文档分享**：支持密码保护和访问次数限制的分享链接
- 🔍 **全文搜索**：基于 MeiliSearch 的快速全文检索
- 📊 **版本管理**：文档版本历史记录
- 👥 **用户系统**：用户注册、登录、JWT 认证

## 🛠️ 技术栈

### 后端
- **Rust** + **Axum** - Web 框架
- **Diesel** - ORM 和数据库管理
- **PostgreSQL** - 关系数据库
- **MinIO** - 对象存储（通过 OpenDAL S3 服务）
- **MeiliSearch** - 全文搜索引擎
- **JWT** - 身份认证
- **bcrypt** - 密码加密

### 前端
- **Vue 3** + **TypeScript**
- **Naive UI** - UI 组件库
- **Pinia** - 状态管理
- **Vue Router** - 路由管理
- **Axios** - HTTP 客户端

### 基础设施
- **Docker Compose** - 本地开发环境
- **OnlyOffice Document Server** - 在线文档编辑

## 🚀 快速开始

### 前置要求

- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose

### 1. 克隆仓库

```bash
git clone https://github.com/YOUR_USERNAME/document-management-system.git
cd document-management-system
```

### 2. 启动基础服务

```bash
docker-compose up -d
```

这将启动：
- PostgreSQL (端口 5432)
- MinIO (端口 9000, 9001)
- MeiliSearch (端口 7700)
- OnlyOffice Document Server (端口 8081)

### 3. 配置后端

创建 `server/.env` 文件：

```env
# 数据库
DATABASE_URL=postgres://dms_user:dms_password@localhost:5432/document_management

# MinIO
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
MINIO_BUCKET=documents
MINIO_REGION=us-east-1

# MeiliSearch
MEILISEARCH_HOST=http://localhost:7700
MEILISEARCH_API_KEY=masterKey

# OnlyOffice
ONLYOFFICE_SERVER=http://localhost:8081
ONLYOFFICE_JWT_SECRET=your-onlyoffice-jwt-secret

# 应用
APP_URL=http://localhost:3000
JWT_SECRET=your-super-secret-jwt-key-change-this-in-production

# 服务器
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

### 4. 运行数据库迁移并启动后端

```bash
cd server
cargo run
```

后端将自动运行数据库迁移并启动在 `http://localhost:3000`

### 5. 启动前端

```bash
cd web
npm install
npm run dev
```

前端将启动在 `http://localhost:5173`

## 📖 API 文档

详细的 API 文档请参考 [API.md](./API.md)

## 🏗️ 项目结构

```
.
├── server/              # Rust 后端
│   ├── src/
│   │   ├── handlers/    # API 处理器
│   │   ├── models/      # 数据模型
│   │   ├── services/    # 业务逻辑
│   │   ├── middleware/  # 中间件
│   │   └── main.rs      # 入口文件
│   ├── migrations/      # 数据库迁移
│   └── Cargo.toml
├── web/                 # Vue 前端
│   ├── src/
│   │   ├── views/       # 页面组件
│   │   ├── components/  # 通用组件
│   │   ├── stores/      # Pinia stores
│   │   ├── api/         # API 调用
│   │   └── router/      # 路由配置
│   └── package.json
└── docker-compose.yml   # Docker 编排
```

## 🔧 开发说明

### 数据库迁移

数据库迁移会在应用启动时自动运行。如需手动运行：

```bash
cd server
diesel migration run
```

### 创建新迁移

```bash
diesel migration generate migration_name
```

## 📝 环境变量说明

详细的环境变量配置请参考 `server/.env.example`

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## 🙏 致谢

- [OnlyOffice](https://www.onlyoffice.com/) - 在线文档编辑
- [Axum](https://github.com/tokio-rs/axum) - Rust Web 框架
- [Naive UI](https://www.naiveui.com/) - Vue UI 组件库
- [MeiliSearch](https://www.meilisearch.com/) - 全文搜索引擎

