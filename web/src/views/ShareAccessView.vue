<template>
  <div class="share-access-view">
    <n-card class="access-card" :loading="loading">
      <template #header>
        <div class="header">
          <h2>分享文档</h2>
        </div>
      </template>

      <!-- 密码验证（如果需要） -->
      <div v-if="needPassword && !verified" class="password-form">
        <n-form @submit.prevent="handleVerifyPassword">
          <n-form-item label="访问密码">
            <n-input
              v-model:value="password"
              type="password"
              placeholder="请输入访问密码"
            />
          </n-form-item>
          <n-form-item>
            <n-button type="primary" @click="handleVerifyPassword">
              验证
            </n-button>
          </n-form-item>
        </n-form>
      </div>

      <!-- 文档信息 -->
      <div v-else-if="shareLink && document" class="document-info">
        <n-descriptions :column="2" bordered>
          <n-descriptions-item label="文件名">
            {{ document.name }}
          </n-descriptions-item>
          <n-descriptions-item label="文件大小">
            {{ formatFileSize(document.file_size) }}
          </n-descriptions-item>
          <n-descriptions-item label="文件类型">
            {{ document.mime_type }}
          </n-descriptions-item>
          <n-descriptions-item label="权限">
            <n-tag :type="shareLink.permission === 'read' ? 'info' : 'success'">
              {{ shareLink.permission === 'read' ? '只读' : '可编辑' }}
            </n-tag>
          </n-descriptions-item>
          <n-descriptions-item label="描述" :span="2">
            {{ document.description || '无' }}
          </n-descriptions-item>
        </n-descriptions>

        <n-space style="margin-top: 20px" justify="center">
          <n-button type="primary" @click="handleDownload">
            <template #icon>
              <n-icon><DownloadOutline /></n-icon>
            </template>
            下载文件
          </n-button>
          <n-button
            v-if="canEdit"
            type="success"
            @click="handleOpenEditor"
          >
            <template #icon>
              <n-icon><CreateOutline /></n-icon>
            </template>
            在线编辑
          </n-button>
        </n-space>

        <!-- OnlyOffice 编辑器 -->
        <div v-if="editorVisible" class="editor-container">
          <div id="share-onlyoffice-editor" style="height: 800px;"></div>
        </div>
      </div>

      <!-- 错误提示 -->
      <n-result
        v-else-if="error"
        status="error"
        :title="error"
        :description="errorDetail"
      >
        <template #footer>
          <n-button type="primary" @click="$router.push('/login')">
            返回登录
          </n-button>
        </template>
      </n-result>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { NCard, NInput, NButton, NIcon, NDescriptions, NDescriptionsItem, NTag, NSpin, useMessage } from 'naive-ui'
import { DownloadOutline, CreateOutline } from '@vicons/ionicons5'
import permissionsApi from '@/api/permissions'
import documentsApi from '@/api/documents'
import onlyofficeApi from '@/api/onlyoffice'
import type { ShareLink, Document } from '@/types'

const route = useRoute()
const message = useMessage()
const token = route.params.token as string

const loading = ref(true)
const needPassword = ref(false)
const verified = ref(false)
const password = ref('')
const shareLink = ref<ShareLink | null>(null)
const document = ref<Document | null>(null)
const error = ref('')
const errorDetail = ref('')
const editorVisible = ref(false)
const canEdit = ref(false)

let editorInstance: any = null

onMounted(async () => {
  await loadShareLink()
})

const loadShareLink = async () => {
  try {
    loading.value = true
    const link = await permissionsApi.getShareLink(token)
    shareLink.value = link

    // 检查是否需要密码
    if (link.password_hash && !verified.value) {
      needPassword.value = true
      loading.value = false
      return
    }

    // 加载文档信息
    await loadDocument()
  } catch (err: any) {
    error.value = '无法访问此分享链接'
    errorDetail.value = err.response?.data?.error || '链接可能已过期或已删除'
  } finally {
    loading.value = false
  }
}

const loadDocument = async () => {
  if (!shareLink.value) return

  try {
    document.value = await documentsApi.get(shareLink.value.document_id)
    canEdit.value = shareLink.value.permission === 'write'
  } catch (err: any) {
    error.value = '无法加载文档'
    errorDetail.value = err.response?.data?.error || '文档不存在或已被删除'
  }
}

const handleVerifyPassword = () => {
  // TODO: 实现密码验证逻辑
  verified.value = true
  needPassword.value = false
  loadDocument()
}

const handleDownload = async () => {
  if (!document.value) return

  try {
    const url = await documentsApi.download(document.value.id)
    window.open(url, '_blank')
  } catch (err: any) {
    message.error(err.response?.data?.error || '下载失败')
  }
}

const handleOpenEditor = async () => {
  if (!document.value) return

  try {
    const config = await onlyofficeApi.getConfig(document.value.id)

    // 加载 OnlyOffice API
    if (!(window as any).DocsAPI) {
      await loadOnlyOfficeScript(config.onlyoffice_server)
    }

    editorVisible.value = true

    // 等待 DOM 更新
    await new Promise(resolve => setTimeout(resolve, 100))

    // 初始化编辑器
    editorInstance = new (window as any).DocsAPI.DocEditor('share-onlyoffice-editor', {
      ...config.config,
      token: config.token,
      events: {
        onDocumentReady: () => {
          console.log('Document is ready')
        },
        onError: (event: any) => {
          console.error('OnlyOffice error:', event)
          message.error('编辑器加载失败')
        }
      }
    })
  } catch (err: any) {
    message.error(err.response?.data?.error || '加载编辑器失败')
  }
}

const loadOnlyOfficeScript = (serverUrl: string): Promise<void> => {
  return new Promise((resolve, reject) => {
    const script = document.createElement('script')
    script.src = `${serverUrl}/web-apps/apps/api/documents/api.js`
    script.onload = () => resolve()
    script.onerror = () => reject(new Error('Failed to load OnlyOffice API'))
    document.head.appendChild(script)
  })
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
.share-access-view {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: var(--n-color-base);
  padding: 20px;
}

.access-card {
  max-width: 1200px;
  width: 100%;
}

.header h2 {
  margin: 0;
  font-size: 24px;
}

.password-form {
  max-width: 400px;
  margin: 40px auto;
}

.document-info {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.editor-container {
  margin-top: 20px;
}
</style>
