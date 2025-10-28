# OnlyOffice JWT 配置指南

## 🔐 JWT 认证工作原理

OnlyOffice Document Server 使用 JWT (JSON Web Token) 来验证和保护配置数据的完整性。

### 工作流程：

1. **后端生成 JWT**：
   - 使用 `JWT_SECRET` 对整个配置对象进行签名
   - 配置对象包括：`document`、`documentType`、`editorConfig`
   
2. **前端接收配置**：
   - 接收包含 `token` 的配置对象
   - 将完整配置（包括 token）传递给 OnlyOffice API

3. **OnlyOffice 验证**：
   - 使用相同的 `JWT_SECRET` 验证 token
   - 确认配置未被篡改

## ⚙️ 配置步骤

### 1. 设置 OnlyOffice Docker 容器的 JWT Secret

编辑 `docker-compose.yml`：

```yaml
onlyoffice:
  image: onlyoffice/documentserver:latest
  container_name: dms_onlyoffice
  environment:
    JWT_ENABLED: "true"
    JWT_SECRET: "your-onlyoffice-jwt-secret"  # 重要：记住这个值
  ports:
    - "8081:80"
```

### 2. 配置后端环境变量

创建 `server/.env` 文件：

```env
# OnlyOffice 配置
ONLYOFFICE_SERVER=http://localhost:8081
ONLYOFFICE_JWT_SECRET=your-onlyoffice-jwt-secret  # 必须与 Docker 中一致！
```

**⚠️ 重要**：`ONLYOFFICE_JWT_SECRET` **必须与** Docker Compose 中的 `JWT_SECRET` **完全一致**！

### 3. 重启所有服务

```bash
# 重启 Docker 容器
docker-compose down
docker-compose up -d

# 重启后端
cd server
cargo run
```

## 📋 完整配置示例

### docker-compose.yml

```yaml
version: '3.8'

services:
  onlyoffice:
    image: onlyoffice/documentserver:9.1.0
    container_name: dms_onlyoffice
    environment:
      JWT_ENABLED: "true"
      JWT_SECRET: "my-super-secret-jwt-key-2024"
    ports:
      - "8081:80"
    volumes:
      - onlyoffice_data:/var/www/onlyoffice/Data
    networks:
      - dms_network
```

### server/.env

```env
# OnlyOffice 配置
ONLYOFFICE_SERVER=http://localhost:8081
ONLYOFFICE_JWT_SECRET=my-super-secret-jwt-key-2024

# 其他配置...
DATABASE_URL=postgres://dms_user:dms_password@localhost:5432/document_management
MINIO_ENDPOINT=http://localhost:9000
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
MINIO_BUCKET=documents
MINIO_REGION=us-east-1
MEILISEARCH_HOST=http://localhost:7700
MEILISEARCH_API_KEY=masterKey
APP_URL=http://localhost:3000
JWT_SECRET=your-app-jwt-secret
SERVER_HOST=127.0.0.1
SERVER_PORT=3000
```

## 🔍 验证配置

### 1. 检查 OnlyOffice 容器

```bash
docker logs dms_onlyoffice
```

应该看到类似：
```
JWT enabled
```

### 2. 测试后端 API

打开浏览器开发者工具，调用文档编辑器配置接口，检查返回的数据结构：

```json
{
  "config": {
    "document": {
      "fileType": "docx",
      "key": "...",
      "title": "...",
      "url": "...",
      "permissions": {...}
    },
    "documentType": "word",
    "editorConfig": {
      "callbackUrl": "...",
      "mode": "edit",
      "user": {...}
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
  },
  "onlyoffice_server": "http://localhost:8081"
}
```

### 3. 验证 Token

Token 应该包含：
- Header: `{"alg":"HS256","typ":"JWT"}`
- Payload: 完整的配置对象（document, documentType, editorConfig）
- Signature: 使用 `ONLYOFFICE_JWT_SECRET` 签名

## 🐛 故障排查

### 问题 1：The document security token is not correctly formed

**原因**：JWT Secret 不匹配

**解决方案**：
1. 确认 `docker-compose.yml` 中的 `JWT_SECRET`
2. 确认 `server/.env` 中的 `ONLYOFFICE_JWT_SECRET`
3. 确保两者完全一致（区分大小写）
4. 重启所有服务

### 问题 2：编辑器一直加载

**原因**：OnlyOffice 服务器未运行或无法访问

**解决方案**：
```bash
# 检查容器状态
docker ps | grep onlyoffice

# 查看容器日志
docker logs dms_onlyoffice

# 测试 OnlyOffice 服务器
curl http://localhost:8081
```

### 问题 3：Token 验证失败

**原因**：Token 签名的内容不正确

**解决方案**：
- 确保后端签名的是传递给 DocEditor 的完整配置对象
- Token 应该包含：`document`、`documentType`、`editorConfig`
- 不要只签名部分配置

## 🔄 代码实现说明

### 后端（Rust）

```rust
// 1. 构建完整配置
let full_config = json!({
    "document": {...},
    "documentType": "word",
    "editorConfig": {...}
});

// 2. 使用 JWT_SECRET 签名
let token = encode(
    &Header::default(),
    &full_config,
    &EncodingKey::from_secret(jwt_secret.as_bytes()),
)?;

// 3. 返回配置和 token
json!({
    "config": {
        ...full_config,
        "token": token
    },
    "onlyoffice_server": "http://localhost:8081"
})
```

### 前端（Vue + TypeScript）

```typescript
// 1. 获取配置
const response = await onlyofficeApi.getConfig(documentId)

// 2. 传递给 DocumentEditor 组件
<DocumentEditor
  :documentServerUrl="response.onlyoffice_server"
  :config="response.config"  // 已包含 token
/>

// 或者手动初始化
new DocsAPI.DocEditor("placeholder", {
  ...response.config  // document, documentType, editorConfig, token
})
```

## 📚 参考文档

- [OnlyOffice JWT 文档](https://api.onlyoffice.com/editors/signature/)
- [OnlyOffice 配置](https://api.onlyoffice.com/editors/config/)
- [OnlyOffice Docker](https://helpcenter.onlyoffice.com/installation/docs-developer-edition-docker.aspx)

## 🎯 生产环境建议

1. **使用强密钥**：
   - 生成安全的随机密钥
   - 至少 32 个字符
   - 包含大小写字母、数字、特殊字符

2. **保护密钥**：
   - 不要提交到 Git
   - 使用环境变量或密钥管理服务
   - 定期轮换密钥

3. **使用 HTTPS**：
   - 生产环境必须使用 HTTPS
   - 配置 SSL 证书
   - 启用 HSTS

4. **监控和日志**：
   - 记录 JWT 验证失败
   - 监控异常访问
   - 设置告警

