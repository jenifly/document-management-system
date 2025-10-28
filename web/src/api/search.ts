import request from '@/utils/request'
import type { SearchQuery, SearchResult } from '@/types'

export default {
  // 搜索文档
  search(params: SearchQuery): Promise<SearchResult[]> {
    return request.get('/search', { params })
  }
}

