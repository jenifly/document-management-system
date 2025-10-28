<template>
  <div class="search-view">
    <n-card>
      <template #header>
        <h2>文档搜索</h2>
      </template>

      <!-- 搜索表单 -->
      <n-form :model="searchForm">
        <n-grid :x-gap="20" :cols="4">
          <n-grid-item :span="3">
            <n-form-item>
              <n-input
                v-model:value="searchForm.query"
                placeholder="输入关键词搜索文档..."
                clearable
                size="large"
                @keyup.enter="handleSearch"
              >
                <template #prefix>
                  <n-icon><SearchOutline /></n-icon>
                </template>
              </n-input>
            </n-form-item>
          </n-grid-item>
          <n-grid-item>
            <n-button type="primary" size="large" @click="handleSearch" block>
              搜索
            </n-button>
          </n-grid-item>
        </n-grid>

        <!-- 高级筛选 -->
        <n-collapse v-model:expanded-names="activeCollapse">
          <n-collapse-item title="高级筛选" name="filters">
            <n-grid :x-gap="20" :cols="3">
              <n-grid-item>
                <n-form-item label="文件类型">
                  <n-select 
                    v-model:value="searchForm.mimeType" 
                    placeholder="选择文件类型" 
                    clearable
                    :options="mimeTypeOptions"
                  />
                </n-form-item>
              </n-grid-item>
              <n-grid-item>
                <n-form-item label="文件类别">
                  <n-select 
                    v-model:value="searchForm.isFolder" 
                    placeholder="选择类别" 
                    clearable
                    :options="fileCategoryOptions"
                  />
                </n-form-item>
              </n-grid-item>
            </n-grid>
          </n-collapse-item>
        </n-collapse>
      </n-form>

      <!-- 搜索结果 -->
      <div v-if="searched" class="search-results">
        <div class="results-header">
          <span>找到 {{ results.length }} 个结果</span>
        </div>

        <n-data-table
          :loading="loading"
          :columns="columns"
          :data="results"
          :pagination="false"
        />

        <n-empty v-if="results.length === 0" description="未找到匹配的文档" />
      </div>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, h } from 'vue'
import { useRouter } from 'vue-router'
import { 
  NCard,
  NForm,
  NFormItem,
  NGrid,
  NGridItem,
  NInput,
  NButton,
  NCollapse,
  NCollapseItem,
  NSelect,
  NDataTable,
  NEmpty,
  NIcon,
  NTag,
  useMessage,
  type DataTableColumns
} from 'naive-ui'
import { SearchOutline, FolderOpenOutline, DocumentTextOutline } from '@vicons/ionicons5'
import searchApi from '@/api/search'
import type { SearchResult } from '@/types'

const router = useRouter()
const message = useMessage()

const loading = ref(false)
const searched = ref(false)
const results = ref<SearchResult[]>([])
const activeCollapse = ref<string[]>([])

const searchForm = reactive({
  query: '',
  mimeType: '',
  isFolder: false
})

const mimeTypeOptions = [
  { label: 'Word 文档', value: 'application/vnd.openxmlformats-officedocument.wordprocessingml.document' },
  { label: 'Excel 表格', value: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' },
  { label: 'PowerPoint 演示', value: 'application/vnd.openxmlformats-officedocument.presentationml.presentation' },
  { label: 'PDF 文档', value: 'application/pdf' },
  { label: '文本文件', value: 'text/plain' }
]

const fileCategoryOptions = [
  { label: '仅文件', value: false },
  { label: '仅文件夹', value: true }
]

const columns: DataTableColumns<SearchResult> = [
  {
    title: '名称',
    key: 'name',
    render: (row) => {
      return h(
        'div',
        {
          style: { cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '8px' },
          onClick: () => handleOpenDocument(row)
        },
        [
          h(NIcon, { color: row.is_folder ? '#2080f0' : undefined }, 
            { default: () => h(row.is_folder ? FolderOpenOutline : DocumentTextOutline) }
          ),
          h('span', row.name)
        ]
      )
    }
  },
  {
    title: '描述',
    key: 'description',
    width: 300,
    render: (row) => row.description || '-'
  },
  {
    title: '类型',
    key: 'mime_type',
    width: 150,
    render: (row) => getFileTypeLabel(row.mime_type)
  },
  {
    title: '标签',
    key: 'tags',
    width: 200,
    render: (row) => {
      if (!row.tags || row.tags.length === 0) {
        return '-'
      }
      return h('div', { style: { display: 'flex', gap: '4px', flexWrap: 'wrap' } },
        row.tags.map(tag => h(NTag, { size: 'small', type: 'info' }, { default: () => tag }))
      )
    }
  },
  {
    title: '修改时间',
    key: 'updated_at',
    width: 180,
    render: (row) => formatDate(row.updated_at)
  }
]

const handleSearch = async () => {
  if (!searchForm.query.trim()) {
    message.warning('请输入搜索关键词')
    return
  }

  try {
    loading.value = true
    searched.value = true
    
    const params: any = {
      q: searchForm.query,
      limit: 50,
      offset: 0
    }

    if (searchForm.mimeType) {
      params.mime_type = searchForm.mimeType
    }

    if (searchForm.isFolder !== undefined) {
      params.is_folder = searchForm.isFolder
    }

    results.value = await searchApi.search(params)
  } catch (error: any) {
    message.error(error.response?.data?.error || '搜索失败')
  } finally {
    loading.value = false
  }
}

const handleOpenDocument = (doc: SearchResult) => {
  if (doc.is_folder) {
    router.push('/documents?folder=' + doc.id)
  } else {
    router.push('/documents/' + doc.id)
  }
}

const getFileTypeLabel = (mimeType: string): string => {
  const typeMap: Record<string, string> = {
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document': 'Word 文档',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet': 'Excel 表格',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation': 'PowerPoint',
    'application/pdf': 'PDF',
    'text/plain': '文本文件',
    'inode/directory': '文件夹'
  }
  return typeMap[mimeType] || '其他'
}

const formatDate = (timestamp: number): string => {
  return new Date(timestamp * 1000).toLocaleString('zh-CN')
}
</script>

<style scoped>
.search-view {
  max-width: 1400px;
  margin: 0 auto;
}

h2 {
  margin: 0;
  font-size: 24px;
}

.search-results {
  margin-top: 30px;
}

.results-header {
  margin-bottom: 20px;
  font-size: 16px;
  color: var(--n-text-color-2);
}
</style>
