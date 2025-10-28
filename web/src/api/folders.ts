import request from '@/utils/request'
import type { Document, CreateFolderRequest } from '@/types'

export default {
  // 创建文件夹
  create(data: CreateFolderRequest): Promise<Document> {
    return request.post('/folders', data)
  }
}

