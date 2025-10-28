# API 文档

## 基础信息

- **Base URL**: `http://localhost:8080`
- **认证方式**: JWT Bearer Token
- **内容类型**: `application/json` (除文件上传外)

## 认证

所有需要认证的请求都需要在 HTTP Header 中包含：
```
Authorization: Bearer <token>
```

---

## 认证 API

### 1. 用户注册

**端点**: `POST /api/auth/register`

**请求体**:
```json
{
  "username": "user1",
  "email": "user1@example.com",
  "password": "password123",
  "full_name": "User One"
}
```

**验证规则**:
- `username`: 3-100 字符
- `email`: 有效的电子邮件地址
- `password`: 至少 6 字符

**响应**: `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "user1",
  "email": "user1@example.com",
  "full_name": "User One",
  "role": "user",
  "is_active": true,
  "created_at": "2024-01-01T00:00:00"
}
```

### 2. 用户登录

**端点**: `POST /api/auth/login`

**请求体**:
```json
{
  "username": "user1",
  "password": "password123"
}
```

**响应**: `200 OK`
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "user1",
    "email": "user1@example.com",
    "full_name": "User One",
    "role": "user",
    "is_active": true,
    "created_at": "2024-01-01T00:00:00"
  }
}
```

### 3. 获取当前用户信息

**端点**: `GET /api/auth/me`

**需要认证**: 是

**响应**: `200 OK`
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "user1",
  "email": "user1@example.com",
  "full_name": "User One",
  "role": "user",
  "is_active": true,
  "created_at": "2024-01-01T00:00:00"
}
```

---

## 文档管理 API

### 1. 创建文件夹

**端点**: `POST /api/folders`

**需要认证**: 是

**请求体**:
```json
{
  "name": "My Folder",
  "description": "A folder for documents",
  "parent_folder_id": null
}
```

**响应**: `200 OK`
```json
{
  "id": "660e8400-e29b-41d4-a716-446655440000",
  "name": "My Folder",
  "description": "A folder for documents",
  "file_path": "",
  "file_size": 0,
  "mime_type": "inode/directory",
  "version": 1,
  "status": "active",
  "owner_id": "550e8400-e29b-41d4-a716-446655440000",
  "parent_folder_id": null,
  "is_folder": true,
  "tags": null,
  "metadata": null,
  "created_at": "2024-01-01T00:00:00",
  "updated_at": "2024-01-01T00:00:00",
  "deleted_at": null
}
```

### 2. 上传文件

**端点**: `POST /api/documents/upload`

**需要认证**: 是

**内容类型**: `multipart/form-data`

**表单字段**:
- `file` (必需): 文件内容
- `parent_folder_id` (可选): 父文件夹 ID
- `description` (可选): 文件描述
- `tags` (可选): 标签，逗号分隔

**响应**: `200 OK`
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440000",
  "name": "document.pdf",
  "description": "Important document",
  "file_path": "uuid/document.pdf",
  "file_size": 1024000,
  "mime_type": "application/pdf",
  "version": 1,
  "status": "active",
  "owner_id": "550e8400-e29b-41d4-a716-446655440000",
  "parent_folder_id": "660e8400-e29b-41d4-a716-446655440000",
  "is_folder": false,
  "tags": ["important", "finance"],
  "metadata": null,
  "created_at": "2024-01-01T00:00:00",
  "updated_at": "2024-01-01T00:00:00",
  "deleted_at": null
}
```

### 3. 获取文档列表

**端点**: `GET /api/documents`

**需要认证**: 是

**查询参数**:
- `limit` (可选): 返回数量，默认 50
- `offset` (可选): 偏移量，默认 0
- `folder_id` (可选): 文件夹 ID

**响应**: `200 OK`
```json
[
  {
    "id": "770e8400-e29b-41d4-a716-446655440000",
    "name": "document.pdf",
    // ... 其他字段
  }
]
```

### 4. 获取文档详情

**端点**: `GET /api/documents/:id`

**需要认证**: 是

**权限要求**: READ

**响应**: `200 OK`
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440000",
  "name": "document.pdf",
  // ... 完整文档信息
}
```

### 5. 更新文档

**端点**: `PUT /api/documents/:id`

**需要认证**: 是

**权限要求**: WRITE

**请求体**:
```json
{
  "name": "new-name.pdf",
  "description": "Updated description",
  "tags": ["tag1", "tag2"]
}
```

**响应**: `200 OK`
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440000",
  "name": "new-name.pdf",
  // ... 更新后的文档信息
}
```

### 6. 删除文档

**端点**: `DELETE /api/documents/:id`

**需要认证**: 是

**权限要求**: DELETE

**响应**: `200 OK`
```json
{
  "message": "Document deleted successfully"
}
```

### 7. 下载文档

**端点**: `GET /api/documents/:id/download`

**需要认证**: 是

**权限要求**: READ

**响应**: `200 OK`
返回预签名的下载 URL（文本格式）

### 8. 移动文档

**端点**: `POST /api/documents/:id/move`

**需要认证**: 是

**权限要求**: WRITE（源和目标）

**请求体**:
```json
{
  "target_folder_id": "660e8400-e29b-41d4-a716-446655440000"
}
```

**响应**: `200 OK`
```json
{
  "id": "770e8400-e29b-41d4-a716-446655440000",
  "parent_folder_id": "660e8400-e29b-41d4-a716-446655440000",
  // ... 其他字段
}
```

---

## 权限管理 API

### 权限类型

- `read`: 读取权限
- `write`: 写入权限
- `delete`: 删除权限
- `share`: 分享权限
- `admin`: 管理权限（包含所有权限）

### 1. 授予权限

**端点**: `POST /api/documents/:id/permissions`

**需要认证**: 是

**权限要求**: ADMIN

**请求体**:
```json
{
  "user_id": "880e8400-e29b-41d4-a716-446655440000",
  "permission": "read",
  "expires_at": "2024-12-31T23:59:59"
}
```

**响应**: `200 OK`
```json
{
  "id": "990e8400-e29b-41d4-a716-446655440000",
  "document_id": "770e8400-e29b-41d4-a716-446655440000",
  "user_id": "880e8400-e29b-41d4-a716-446655440000",
  "permission": "read",
  "granted_by": "550e8400-e29b-41d4-a716-446655440000",
  "granted_at": "2024-01-01T00:00:00",
  "expires_at": "2024-12-31T23:59:59"
}
```

### 2. 撤销权限

**端点**: `DELETE /api/documents/:document_id/permissions/:user_id/:permission`

**需要认证**: 是

**权限要求**: ADMIN

**响应**: `200 OK`
```json
{
  "message": "Permission revoked successfully"
}
```

### 3. 获取文档权限列表

**端点**: `GET /api/documents/:id/permissions`

**需要认证**: 是

**权限要求**: ADMIN

**响应**: `200 OK`
```json
[
  {
    "id": "990e8400-e29b-41d4-a716-446655440000",
    "document_id": "770e8400-e29b-41d4-a716-446655440000",
    "user_id": "880e8400-e29b-41d4-a716-446655440000",
    "permission": "read",
    "granted_by": "550e8400-e29b-41d4-a716-446655440000",
    "granted_at": "2024-01-01T00:00:00",
    "expires_at": null
  }
]
```

### 4. 创建分享链接

**端点**: `POST /api/documents/:id/share`

**需要认证**: 是

**权限要求**: SHARE

**请求体**:
```json
{
  "permission": "read",
  "password": "optional_password",
  "max_access_count": 10,
  "expires_at": "2024-12-31T23:59:59"
}
```

**响应**: `200 OK`
```json
{
  "id": "aa0e8400-e29b-41d4-a716-446655440000",
  "document_id": "770e8400-e29b-41d4-a716-446655440000",
  "token": "unique-token-string",
  "created_by": "550e8400-e29b-41d4-a716-446655440000",
  "permission": "read",
  "password_hash": null,
  "max_access_count": 10,
  "access_count": 0,
  "expires_at": "2024-12-31T23:59:59",
  "created_at": "2024-01-01T00:00:00"
}
```

### 5. 访问分享链接

**端点**: `GET /api/share/:token`

**需要认证**: 否

**响应**: `200 OK`
返回分享链接信息（自动增加访问计数）

### 6. 删除分享链接

**端点**: `DELETE /api/share/:id`

**需要认证**: 是

**权限要求**: 创建者或文档管理员

**响应**: `200 OK`
```json
{
  "message": "Share link deleted successfully"
}
```

---

## 搜索 API

### 搜索文档

**端点**: `GET /api/search`

**需要认证**: 是

**查询参数**:
- `q` (必需): 搜索关键词
- `limit` (可选): 返回数量，默认 50
- `offset` (可选): 偏移量，默认 0
- `owner_id` (可选): 按所有者筛选
- `mime_type` (可选): 按 MIME 类型筛选
- `is_folder` (可选): 是否为文件夹

**示例**:
```
GET /api/search?q=report&limit=10&mime_type=application/pdf
```

**响应**: `200 OK`
```json
[
  {
    "id": "770e8400-e29b-41d4-a716-446655440000",
    "name": "annual-report.pdf",
    "description": "Annual financial report",
    "mime_type": "application/pdf",
    "owner_id": "550e8400-e29b-41d4-a716-446655440000",
    "is_folder": false,
    "tags": ["finance", "report"],
    "created_at": 1704067200,
    "updated_at": 1704067200
  }
]
```

---

## OnlyOffice API

### 1. 获取编辑器配置

**端点**: `GET /api/onlyoffice/:id/config`

**需要认证**: 是

**权限要求**: READ（查看），WRITE（编辑）

**响应**: `200 OK`
```json
{
  "config": {
    "document": {
      "file_type": "docx",
      "key": "770e8400-e29b-41d4-a716-446655440000",
      "title": "document.docx",
      "url": "https://presigned-url...",
      "permissions": {
        "download": true,
        "edit": true,
        "print": true,
        "review": true,
        "comment": true
      }
    },
    "editor_config": {
      "callback_url": "http://localhost:8080/api/onlyoffice/callback/770e8400-...",
      "mode": "edit",
      "user": {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "name": "user1"
      }
    }
  },
  "token": "jwt-token-for-onlyoffice",
  "onlyoffice_server": "http://localhost:8081"
}
```

### 2. OnlyOffice 回调

**端点**: `POST /api/onlyoffice/callback/:id`

**需要认证**: 否（由 OnlyOffice 服务器调用）

**请求体**:
```json
{
  "key": "770e8400-e29b-41d4-a716-446655440000",
  "status": 2,
  "url": "https://onlyoffice-server/download-url",
  "users": ["user1"]
}
```

**状态码**:
- 0: 文档正在编辑
- 1: 文档正在编辑，当前用户已关闭文档
- 2: 文档准备保存
- 3: 文档保存错误
- 4: 文档已关闭，无更改
- 6: 文档正在编辑，保存超时
- 7: 强制保存时发生错误

**响应**: `200 OK`
```json
{
  "error": 0,
  "message": "Document saved",
  "download_url": "https://..."
}
```

---

## 错误响应

所有错误响应格式统一：

```json
{
  "error": "错误描述信息"
}
```

### HTTP 状态码

- `200`: 成功
- `400`: 请求错误
- `401`: 未认证
- `403`: 权限不足
- `404`: 资源不存在
- `500`: 服务器内部错误

### 常见错误

**401 Unauthorized**:
```json
{
  "error": "Missing authorization header"
}
```

**403 Forbidden**:
```json
{
  "error": "No permission to access this resource"
}
```

**404 Not Found**:
```json
{
  "error": "Resource not found"
}
```

**400 Bad Request**:
```json
{
  "error": "Validation error: username must be at least 3 characters"
}
```

---

## 使用示例

### cURL 示例

#### 注册用户
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "email": "test@example.com",
    "password": "password123",
    "full_name": "Test User"
  }'
```

#### 登录
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "testuser",
    "password": "password123"
  }'
```

#### 上传文件
```bash
curl -X POST http://localhost:8080/api/documents/upload \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -F "file=@/path/to/document.pdf" \
  -F "description=My document" \
  -F "tags=important,work"
```

#### 搜索文档
```bash
curl -X GET "http://localhost:8080/api/search?q=report&limit=10" \
  -H "Authorization: Bearer YOUR_TOKEN"
```

### JavaScript 示例

```javascript
// 登录
const login = async () => {
  const response = await fetch('http://localhost:8080/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      username: 'testuser',
      password: 'password123',
    }),
  });
  
  const data = await response.json();
  return data.token;
};

// 上传文件
const uploadFile = async (token, file) => {
  const formData = new FormData();
  formData.append('file', file);
  formData.append('description', 'Uploaded from browser');
  
  const response = await fetch('http://localhost:8080/api/documents/upload', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${token}`,
    },
    body: formData,
  });
  
  return await response.json();
};

// 搜索文档
const searchDocuments = async (token, query) => {
  const response = await fetch(`http://localhost:8080/api/search?q=${encodeURIComponent(query)}`, {
    headers: {
      'Authorization': `Bearer ${token}`,
    },
  });
  
  return await response.json();
};
```

