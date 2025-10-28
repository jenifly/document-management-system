<template>
  <div class="home-view">
    <n-space vertical :size="20">
      <!-- 欢迎卡片 -->
      <n-card>
        <h2>欢迎回来，{{ userStore.user?.full_name || userStore.user?.username }}！</h2>
        <p>这是您的文档管理中心，您可以在这里管理您的所有文档。</p>
      </n-card>

      <!-- 统计卡片 -->
      <n-grid :x-gap="20" :y-gap="20" :cols="3">
        <n-grid-item>
          <n-card>
            <n-statistic label="总文档数" :value="stats.totalDocuments">
              <template #prefix>
                <n-icon size="40" color="#18a058">
                  <DocumentTextOutline />
                </n-icon>
              </template>
            </n-statistic>
          </n-card>
        </n-grid-item>

        <n-grid-item>
          <n-card>
            <n-statistic label="文件夹数" :value="stats.totalFolders">
              <template #prefix>
                <n-icon size="40" color="#2080f0">
                  <FolderOpenOutline />
                </n-icon>
              </template>
            </n-statistic>
          </n-card>
        </n-grid-item>

        <n-grid-item>
          <n-card>
            <n-statistic label="分享数" :value="stats.totalShares">
              <template #prefix>
                <n-icon size="40" color="#f0a020">
                  <ShareSocialOutline />
                </n-icon>
              </template>
            </n-statistic>
          </n-card>
        </n-grid-item>
      </n-grid>

      <!-- 快速操作 -->
      <n-card title="快速操作">
        <n-space>
          <n-button type="primary" @click="router.push('/documents')">
            <template #icon>
              <n-icon><CloudUploadOutline /></n-icon>
            </template>
            上传文档
          </n-button>
          <n-button type="success" @click="router.push('/documents')">
            <template #icon>
              <n-icon><FolderOpenOutline /></n-icon>
            </template>
            新建文件夹
          </n-button>
          <n-button type="warning" @click="router.push('/search')">
            <template #icon>
              <n-icon><SearchOutline /></n-icon>
            </template>
            搜索文档
          </n-button>
        </n-space>
      </n-card>

      <!-- 最近文档 -->
      <n-card title="最近文档">
        <template #header-extra>
          <n-button text @click="router.push('/documents')">
            查看全部 →
          </n-button>
        </template>
        <n-data-table
          :loading="loading"
          :columns="columns"
          :data="recentDocuments"
          :pagination="false"
        />
      </n-card>
    </n-space>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import { 
  NCard, 
  NSpace, 
  NGrid, 
  NGridItem, 
  NStatistic, 
  NButton, 
  NDataTable,
  NIcon,
  useMessage,
  type DataTableColumns
} from 'naive-ui'
import {
  DocumentTextOutline,
  FolderOpenOutline,
  ShareSocialOutline,
  CloudUploadOutline,
  SearchOutline
} from '@vicons/ionicons5'
import { useUserStore } from '@/stores/user'
import { useDocumentStore } from '@/stores/document'
import type { Document as DocumentType } from '@/types'

const router = useRouter()
const userStore = useUserStore()
const documentStore = useDocumentStore()
const message = useMessage()

const loading = ref(false)
const recentDocuments = ref<DocumentType[]>([])

const stats = reactive({
  totalDocuments: 0,
  totalFolders: 0,
  totalShares: 0
})

const columns: DataTableColumns<DocumentType> = [
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
    title: '大小',
    key: 'file_size',
    width: 120,
    render: (row) => row.is_folder ? '-' : formatFileSize(row.file_size)
  },
  {
    title: '修改时间',
    key: 'updated_at',
    width: 180,
    render: (row) => formatDate(row.updated_at)
  }
]

onMounted(async () => {
  await loadDashboardData()
})

const loadDashboardData = async () => {
  try {
    loading.value = true
    const documents = await documentStore.fetchDocuments()
    
    // 计算统计数据
    stats.totalDocuments = documents.filter(d => !d.is_folder).length
    stats.totalFolders = documents.filter(d => d.is_folder).length
    stats.totalShares = 0 // TODO: 添加分享数统计
    
    // 获取最近的 5 个文档
    recentDocuments.value = documents
      .sort((a, b) => new Date(b.updated_at).getTime() - new Date(a.updated_at).getTime())
      .slice(0, 5)
  } catch (error: any) {
    message.error(error.response?.data?.error || '加载数据失败')
  } finally {
    loading.value = false
  }
}

const handleOpenDocument = (doc: DocumentType) => {
  if (doc.is_folder) {
    router.push('/documents?folder=' + doc.id)
  } else {
    router.push('/documents/' + doc.id)
  }
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}

const formatDate = (dateStr: string): string => {
  return new Date(dateStr).toLocaleString('zh-CN')
}
</script>

<style scoped>
.home-view {
  max-width: 1400px;
  margin: 0 auto;
}

h2 {
  margin: 0 0 10px 0;
  font-size: 24px;
}

p {
  margin: 0;
  color: var(--n-text-color-2);
}
</style>
