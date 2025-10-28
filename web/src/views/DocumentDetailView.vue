<template>
  <div class="document-detail-view">
    <n-card v-if="loading || !currentDocument">
      <n-skeleton text :repeat="10" />
    </n-card>

    <n-space v-else vertical :size="20">
      <!-- 文档信息栏 -->
      <n-card>
        <template #header>
          <div class="card-header">
            <div class="title-section">
              <n-icon size="24"><DocumentTextOutline /></n-icon>
              <h2>{{ currentDocument.name }}</h2>
            </div>
            <n-space>
              <n-button @click="handleDownload">
                <template #icon><n-icon><DownloadOutline /></n-icon></template>
                下载
              </n-button>
              <n-button @click="showShareDialog = true">
                <template #icon><n-icon><ShareSocialOutline /></n-icon></template>
                分享
              </n-button>
              <n-button @click="showPermissionDialog = true">
                <template #icon><n-icon><SettingsOutline /></n-icon></template>
                权限
              </n-button>
              <n-button @click="showEditDialog = true">
                <template #icon><n-icon><CreateOutline /></n-icon></template>
                编辑信息
              </n-button>
            </n-space>
          </div>
        </template>

        <n-descriptions :column="2" bordered>
          <n-descriptions-item label="文件大小">
            {{ formatFileSize(currentDocument.file_size) }}
          </n-descriptions-item>
          <n-descriptions-item label="文件类型">
            {{ currentDocument.mime_type }}
          </n-descriptions-item>
          <n-descriptions-item label="版本">
            v{{ currentDocument.version }}
          </n-descriptions-item>
          <n-descriptions-item label="状态">
            <n-tag type="success">{{ currentDocument.status }}</n-tag>
          </n-descriptions-item>
          <n-descriptions-item label="创建时间">
            {{ formatDate(currentDocument.created_at) }}
          </n-descriptions-item>
          <n-descriptions-item label="修改时间">
            {{ formatDate(currentDocument.updated_at) }}
          </n-descriptions-item>
          <n-descriptions-item label="描述" :span="2">
            {{ currentDocument.description || '无' }}
          </n-descriptions-item>
          <n-descriptions-item label="标签" :span="2">
            <n-space v-if="currentDocument.tags && currentDocument.tags.length > 0">
              <n-tag
                v-for="tag in currentDocument.tags"
                :key="tag"
                type="info"
                size="small"
              >
                {{ tag }}
              </n-tag>
            </n-space>
            <span v-else>无</span>
          </n-descriptions-item>
        </n-descriptions>
      </n-card>

      <!-- OnlyOffice 编辑器 -->
      <n-card v-if="canEdit">
        <template #header>
          <div class="card-header">
            <h3>文档编辑器</h3>
            <n-space>
              <n-button
                v-if="!editorLoaded"
                type="primary"
                :loading="loadingEditor"
                @click="loadEditor"
              >
                在此页面打开
              </n-button>
              <n-button
                type="primary"
                @click="openInNewTab"
              >
                在新标签页打开
              </n-button>
            </n-space>
          </div>
        </template>

        <div v-if="editorLoaded" id="onlyoffice-editor" style="height: 800px;"></div>
        <n-empty v-else description="点击上方按钮加载编辑器" />
      </n-card>

      <n-card v-else>
        <n-empty description="您没有编辑此文档的权限" />
      </n-card>
    </n-space>

    <!-- 编辑文档信息对话框 -->
    <n-modal v-model:show="showEditDialog" preset="dialog" title="编辑文档信息">
      <n-form :model="editForm" label-placement="left" label-width="80">
        <n-form-item label="文件名">
          <n-input v-model:value="editForm.name" />
        </n-form-item>
        <n-form-item label="描述">
          <n-input
            v-model:value="editForm.description"
            type="textarea"
            :rows="3"
          />
        </n-form-item>
        <n-form-item label="标签">
          <n-input
            v-model:value="editForm.tags"
            placeholder="标签，用逗号分隔"
          />
        </n-form-item>
      </n-form>
      <template #action>
        <n-space>
          <n-button @click="showEditDialog = false">取消</n-button>
          <n-button type="primary" @click="handleUpdateDocument">保存</n-button>
        </n-space>
      </template>
    </n-modal>

    <!-- 权限管理对话框 -->
    <PermissionDialog
      v-model="showPermissionDialog"
      :document-id="documentId"
    />

    <!-- 分享对话框 -->
    <ShareDialog
      v-model="showShareDialog"
      :document-id="documentId"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { 
  NCard, 
  NSpace, 
  NButton, 
  NIcon, 
  NDescriptions, 
  NDescriptionsItem, 
  NTag, 
  NSkeleton,
  NModal,
  NForm,
  NFormItem,
  NInput,
  NEmpty,
  useMessage 
} from 'naive-ui'
import {
  DocumentTextOutline,
  DownloadOutline,
  ShareSocialOutline,
  SettingsOutline,
  CreateOutline
} from '@vicons/ionicons5'
import { useDocumentStore } from '@/stores/document'
import onlyofficeApi from '@/api/onlyoffice'
import type { Document as DocumentType } from '@/types'
import PermissionDialog from '@/components/PermissionDialog.vue'
import ShareDialog from '@/components/ShareDialog.vue'

const route = useRoute()
const router = useRouter()
const documentStore = useDocumentStore()
const message = useMessage()

const documentId = route.params.id as string
const currentDocument = ref<DocumentType | null>(null)
const loading = ref(true)
const loadingEditor = ref(false)
const editorLoaded = ref(false)
const canEdit = ref(false)
const showEditDialog = ref(false)
const showPermissionDialog = ref(false)
const showShareDialog = ref(false)

const editForm = reactive({
  name: '',
  description: '',
  tags: ''
})

let editorInstance: any = null

onMounted(async () => {
  await loadDocument()
  checkEditPermission()
})

onUnmounted(() => {
  if (editorInstance) {
    editorInstance.destroyEditor()
  }
})

const loadDocument = async () => {
  try {
    loading.value = true
    currentDocument.value = await documentStore.getDocument(documentId)
    
    // 初始化编辑表单
    editForm.name = currentDocument.value.name
    editForm.description = currentDocument.value.description || ''
    editForm.tags = currentDocument.value.tags?.join(', ') || ''
  } catch (error: any) {
    message.error(error.response?.data?.error || '加载文档失败')
  } finally {
    loading.value = false
  }
}

const checkEditPermission = () => {
  if (!currentDocument.value) return
  
  // 检查文件类型是否支持编辑
  const editableMimeTypes = [
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
    'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
    'application/vnd.openxmlformats-officedocument.presentationml.presentation',
    'text/plain'
  ]
  
  canEdit.value = editableMimeTypes.includes(currentDocument.value.mime_type)
}

const loadEditor = async () => {
  if (!currentDocument.value) return
  
  try {
    loadingEditor.value = true
    console.log('开始加载编辑器配置...')
    const config = await onlyofficeApi.getConfig(documentId)
    console.log('OnlyOffice 配置:', config)
    
    // 加载 OnlyOffice API
    if (!(window as any).DocsAPI) {
      console.log('加载 OnlyOffice 脚本...')
      await loadOnlyOfficeScript(config.onlyoffice_server)
      console.log('OnlyOffice 脚本加载完成')
    }
    
    // 初始化编辑器
    console.log('初始化编辑器...')
    
    // OnlyOffice API 配置
    // config.config 已经包含了 document, documentType, editorConfig 和 token
    const editorConfig = {
      ...config.config,
      events: {
        onDocumentReady: () => {
          console.log('Document is ready')
        },
        onError: (event: any) => {
          console.error('OnlyOffice error:', event)
          message.error(`编辑器错误: ${JSON.stringify(event)}`)
        }
      }
    }
    
    console.log('Editor config:', editorConfig)
    
    editorInstance = new (window as any).DocsAPI.DocEditor('onlyoffice-editor', editorConfig)
    
    editorLoaded.value = true
    console.log('编辑器初始化完成')
  } catch (error: any) {
    console.error('加载编辑器失败:', error)
    console.error('错误详情:', {
      message: error.message,
      response: error.response,
      status: error.response?.status,
      data: error.response?.data
    })
    const errorMsg = error.response?.data?.error || error.message || '加载编辑器失败'
    message.error(errorMsg)
  } finally {
    loadingEditor.value = false
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

const openInNewTab = () => {
  // 在新标签页打开编辑器
  const route = router.resolve({ name: 'editor', params: { id: documentId } })
  window.open(route.href, '_blank')
}

const handleDownload = async () => {
  try {
    const url = await documentStore.downloadDocument(documentId)
    window.open(url, '_blank')
  } catch (error: any) {
    message.error(error.response?.data?.error || '下载失败')
  }
}

const handleUpdateDocument = async () => {
  if (!editForm.name) {
    message.warning('文件名不能为空')
    return
  }
  
  try {
    const tags = editForm.tags
      .split(',')
      .map(t => t.trim())
      .filter(t => t.length > 0)
    
    await documentStore.updateDocument(documentId, {
      name: editForm.name,
      description: editForm.description || undefined,
      tags: tags.length > 0 ? tags : undefined
    })
    
    currentDocument.value = await documentStore.getDocument(documentId)
    showEditDialog.value = false
    message.success('更新成功')
  } catch (error: any) {
    message.error(error.response?.data?.error || '更新失败')
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
.document-detail-view {
  max-width: 1400px;
  margin: 0 auto;
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.title-section {
  display: flex;
  align-items: center;
  gap: 12px;
}

.title-section h2 {
  margin: 0;
  font-size: 20px;
}

.title-section h3 {
  margin: 0;
  font-size: 18px;
}

#onlyoffice-editor {
  width: 100%;
  min-height: 600px;
}
</style>
