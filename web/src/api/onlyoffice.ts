import request from '@/utils/request'
import type { OnlyOfficeConfig } from '@/types'

export default {
  // 获取 OnlyOffice 编辑器配置
  getConfig(documentId: string): Promise<OnlyOfficeConfig> {
    return request.get(`/onlyoffice/${documentId}/config`)
  }
}

