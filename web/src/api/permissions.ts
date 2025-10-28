import request from '@/utils/request'
import type {
  DocumentPermission,
  GrantPermissionRequest,
  CreateShareLinkRequest,
  ShareLink,
  PermissionType
} from '@/types'

export default {
  // 获取文档权限列表
  list(documentId: string): Promise<DocumentPermission[]> {
    return request.get(`/documents/${documentId}/permissions`)
  },

  // 授予权限
  grant(documentId: string, data: GrantPermissionRequest): Promise<DocumentPermission> {
    return request.post(`/documents/${documentId}/permissions`, data)
  },

  // 撤销权限
  revoke(documentId: string, userId: string, permission: PermissionType): Promise<void> {
    return request.delete(`/documents/${documentId}/permissions/${userId}/${permission}`)
  },

  // 创建分享链接
  createShareLink(documentId: string, data: CreateShareLinkRequest): Promise<ShareLink> {
    return request.post(`/documents/${documentId}/share`, data)
  },

  // 获取分享链接（公开访问）
  getShareLink(token: string): Promise<ShareLink> {
    return request.get(`/share/${token}`)
  },

  // 删除分享链接
  deleteShareLink(linkId: string): Promise<void> {
    return request.delete(`/share/${linkId}`)
  }
}
