# 文档管理系统 - 前端

基于 Vue 3 + TypeScript + Element Plus 构建的现代化文档管理系统前端。

## 技术栈

- **框架**: Vue 3 (Composition API)
- **语言**: TypeScript
- **构建工具**: Vite
- **UI 框架**: Element Plus
- **状态管理**: Pinia
- **路由**: Vue Router
- **HTTP 客户端**: Axios

## 项目结构

```
web/
├── public/              # 静态资源
├── src/
│   ├── api/            # API 接口
│   ├── assets/         # 资源文件
│   ├── components/     # 通用组件
│   ├── router/         # 路由配置
│   ├── stores/         # Pinia 状态管理
│   ├── types/          # TypeScript 类型定义
│   ├── utils/          # 工具函数
│   ├── views/          # 页面组件
│   ├── App.vue         # 根组件
│   ├── main.ts         # 入口文件
│   └── env.d.ts        # 环境变量类型定义
├── index.html          # HTML 模板
├── vite.config.ts      # Vite 配置
├── tsconfig.json       # TypeScript 配置
└── package.json        # 项目依赖

```

## 快速开始

### 安装依赖

```bash
npm install
# 或
yarn install
# 或
pnpm install
```

### 开发

```bash
npm run dev
```

访问 http://localhost:3000

### 构建

```bash
npm run build
```

### 预览构建结果

```bash
npm run preview
```

### 类型检查

```bash
npm run type-check
```

## 功能特性

- ✅ 用户认证（登录/注册）
- ✅ 文档管理（上传/下载/删除）
- ✅ 文件夹组织
- ✅ 文档搜索
- ✅ 权限管理
- ✅ 分享链接
- ✅ OnlyOffice 在线编辑
- ✅ 响应式设计

## 开发规范

### 代码风格

- 使用 TypeScript 严格模式
- 使用 Composition API
- 使用 `<script setup>` 语法
- 遵循 Vue 3 最佳实践

### 命名规范

- 组件名: PascalCase (`UserProfile.vue`)
- 文件名: kebab-case (`user-profile.ts`)
- 变量名: camelCase (`userName`)
- 常量名: UPPER_SNAKE_CASE (`API_BASE_URL`)

### 目录规范

- `views/`: 页面级组件
- `components/`: 可复用组件
- `api/`: API 接口按模块分类
- `stores/`: 状态管理按模块分类
- `types/`: 类型定义集中管理

## 环境变量

创建 `.env.local` 文件配置本地环境变量：

```env
VITE_API_BASE_URL=http://localhost:8080/api
```

## API 代理

开发环境通过 Vite 代理转发 API 请求到后端：

```typescript
// vite.config.ts
proxy: {
  '/api': {
    target: 'http://localhost:8080',
    changeOrigin: true
  }
}
```

## 路由

- `/login` - 登录页
- `/register` - 注册页
- `/` - 首页
- `/documents` - 文档列表
- `/documents/:id` - 文档详情
- `/search` - 搜索页
- `/shared` - 分享管理
- `/profile` - 个人资料
- `/share/:token` - 分享访问（无需登录）

## 状态管理

使用 Pinia 进行状态管理：

- `user`: 用户信息和认证状态
- `document`: 文档列表和操作

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 许可证

MIT License

