// 用户相关类型
export interface User {
  id: string
  username: string
  email: string
  full_name?: string
  role: string
  is_active: boolean
  created_at: string
}

export interface LoginRequest {
  username: string
  password: string
}

export interface RegisterRequest {
  username: string
  email: string
  password: string
  full_name?: string
}

export interface LoginResponse {
  token: string
  user: User
}

// 文档相关类型
export interface Document {
  id: string
  name: string
  description?: string
  file_path: string
  file_size: number
  mime_type: string
  version: number
  status: string
  owner_id: string
  parent_folder_id?: string
  is_folder: boolean
  tags?: string[]
  metadata?: Record<string, any>
  created_at: string
  updated_at: string
  deleted_at?: string
}

export interface CreateFolderRequest {
  name: string
  description?: string
  parent_folder_id?: string
}

export interface UpdateDocumentRequest {
  name?: string
  description?: string
  tags?: string[]
}

export interface MoveDocumentRequest {
  target_folder_id?: string
}

// 权限相关类型
export type PermissionType = 'read' | 'write' | 'delete' | 'share' | 'admin'

export interface DocumentPermission {
  id: string
  document_id: string
  user_id: string
  permission: PermissionType
  granted_by: string
  granted_at: string
  expires_at?: string
}

export interface GrantPermissionRequest {
  user_id: string
  permission: PermissionType
  expires_at?: string
}

export interface CreateShareLinkRequest {
  permission: PermissionType
  password?: string
  max_access_count?: number
  expires_at?: string
}

export interface ShareLink {
  id: string
  document_id: string
  token: string
  created_by: string
  permission: PermissionType
  password_hash?: string
  max_access_count?: number
  access_count: number
  expires_at?: string
  created_at: string
}

// 搜索相关类型
export interface SearchQuery {
  q: string
  limit?: number
  offset?: number
  owner_id?: string
  mime_type?: string
  is_folder?: boolean
}

export interface SearchResult {
  id: string
  name: string
  description?: string
  mime_type: string
  owner_id: string
  is_folder: boolean
  tags: string[]
  created_at: number
  updated_at: number
}

// OnlyOffice 相关类型
export interface OnlyOfficeConfig {
  config: {
    document: {
      file_type: string
      key: string
      title: string
      url: string
      permissions: {
        download: boolean
        edit: boolean
        print: boolean
        review: boolean
        comment: boolean
      }
    }
    editor_config: {
      callback_url: string
      mode: string
      user: {
        id: string
        name: string
      }
    }
  }
  token: string
  onlyoffice_server: string
}

// API 响应类型
export interface ApiResponse<T = any> {
  data?: T
  error?: string
  message?: string
}

// 分页参数
export interface PaginationParams {
  limit?: number
  offset?: number
}

// 上传进度
export interface UploadProgress {
  loaded: number
  total: number
  percentage: number
}

