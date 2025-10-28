<template>
  <div class="profile-view">
    <n-grid :x-gap="20" :cols="3">
      <n-grid-item :span="2">
        <n-card title="个人信息">
          <n-descriptions v-if="userStore.user" :column="1" bordered>
            <n-descriptions-item label="用户名">
              {{ userStore.user.username }}
            </n-descriptions-item>
            <n-descriptions-item label="邮箱">
              {{ userStore.user.email }}
            </n-descriptions-item>
            <n-descriptions-item label="全名">
              {{ userStore.user.full_name || '-' }}
            </n-descriptions-item>
            <n-descriptions-item label="角色">
              <n-tag :type="getRoleType(userStore.user.role)">
                {{ getRoleLabel(userStore.user.role) }}
              </n-tag>
            </n-descriptions-item>
            <n-descriptions-item label="状态">
              <n-tag :type="userStore.user.is_active ? 'success' : 'error'">
                {{ userStore.user.is_active ? '激活' : '未激活' }}
              </n-tag>
            </n-descriptions-item>
            <n-descriptions-item label="注册时间">
              {{ formatDate(userStore.user.created_at) }}
            </n-descriptions-item>
          </n-descriptions>
        </n-card>
      </n-grid-item>

      <n-grid-item>
        <n-space vertical :size="20">
          <n-card title="账户统计">
            <n-space vertical>
              <div class="stat-item">
                <div class="stat-label">总文档数</div>
                <div class="stat-value">{{ stats.documents }}</div>
              </div>
              <n-divider style="margin: 12px 0" />
              <div class="stat-item">
                <div class="stat-label">总文件夹数</div>
                <div class="stat-value">{{ stats.folders }}</div>
              </div>
              <n-divider style="margin: 12px 0" />
              <div class="stat-item">
                <div class="stat-label">总存储空间</div>
                <div class="stat-value">{{ formatFileSize(stats.totalSize) }}</div>
              </div>
            </n-space>
          </n-card>

          <n-card title="快捷操作">
            <n-space vertical>
              <n-button type="primary" @click="router.push('/documents')" block>
                我的文档
              </n-button>
              <n-button type="success" @click="router.push('/search')" block>
                搜索文档
              </n-button>
            </n-space>
          </n-card>
        </n-space>
      </n-grid-item>
    </n-grid>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { NCard, NDescriptions, NDescriptionsItem, NTag, NGrid, NGridItem, NButton, useMessage } from 'naive-ui'
import { useUserStore } from '@/stores/user'
import { useDocumentStore } from '@/stores/document'

const router = useRouter()
const userStore = useUserStore()
const documentStore = useDocumentStore()
const message = useMessage()

const stats = reactive({
  documents: 0,
  folders: 0,
  totalSize: 0
})

onMounted(async () => {
  await loadStats()
})

const loadStats = async () => {
  try {
    const documents = await documentStore.fetchDocuments()
    
    stats.documents = documents.filter(d => !d.is_folder).length
    stats.folders = documents.filter(d => d.is_folder).length
    stats.totalSize = documents
      .filter(d => !d.is_folder)
      .reduce((sum, d) => sum + d.file_size, 0)
  } catch (error: any) {
    message.error(error.response?.data?.error || '加载统计数据失败')
  }
}

const getRoleLabel = (role: string): string => {
  const roleMap: Record<string, string> = {
    admin: '管理员',
    user: '普通用户'
  }
  return roleMap[role] || role
}

const getRoleType = (role: string): 'error' | 'success' => {
  return role === 'admin' ? 'error' : 'success'
}

const formatDate = (dateStr: string): string => {
  return new Date(dateStr).toLocaleString('zh-CN')
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i]
}
</script>

<style scoped>
.profile-view {
  max-width: 1400px;
  margin: 0 auto;
}

.stat-item {
  text-align: center;
}

.stat-label {
  font-size: 14px;
  color: var(--n-text-color-3);
  margin-bottom: 8px;
}

.stat-value {
  font-size: 28px;
  font-weight: bold;
  color: var(--n-text-color-1);
}
</style>
