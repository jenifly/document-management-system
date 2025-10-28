import request from '@/utils/request'
import type { Document, UpdateDocumentRequest, MoveDocumentRequest, PaginationParams } from '@/types'

export default {
  // 获取文档列表
  list(params?: PaginationParams & { folder_id?: string }): Promise<Document[]> {
    return request.get('/documents', { params })
  },

  // 获取文档详情
  get(id: string): Promise<Document> {
    return request.get(`/documents/${id}`)
  },

  // 上传文件
  upload(formData: FormData): Promise<Document> {
    return request.post('/documents/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    })
  },

  // 更新文档
  update(id: string, data: UpdateDocumentRequest): Promise<Document> {
    return request.put(`/documents/${id}`, data)
  },

  // 删除文档
  delete(id: string): Promise<{ message: string }> {
    return request.delete(`/documents/${id}`)
  },

  // 下载文档（获取预签名 URL）
  download(id: string): Promise<string> {
    return request.get(`/documents/${id}/download`)
  },

  // 移动文档
  move(id: string, data: MoveDocumentRequest): Promise<Document> {
    return request.post(`/documents/${id}/move`, data)
  }
}

