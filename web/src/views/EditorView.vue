<template>
  <div class="editor-view">
    <div v-if="loading" class="loading">
      <n-spin size="large">
        <template #description>正在加载编辑器...</template>
      </n-spin>
    </div>
    <div v-else-if="error" class="error">
      <n-result status="error" :title="error" description="请返回重试">
        <template #footer>
          <n-button @click="router.back()">返回</n-button>
        </template>
      </n-result>
    </div>
    <DocumentEditor
      v-else
      id="docEditor"
      :documentServerUrl="documentServerUrl"
      :config="editorConfig"
      :events_onDocumentReady="onDocumentReady"
      :onLoadComponentError="onLoadComponentError"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { NSpin, NResult, NButton, useMessage } from 'naive-ui'
import { DocumentEditor } from '@onlyoffice/document-editor-vue'
import onlyofficeApi from '@/api/onlyoffice'

const route = useRoute()
const router = useRouter()
const message = useMessage()

const documentId = route.params.id as string
const loading = ref(true)
const error = ref('')
const documentServerUrl = ref('')
const editorConfig = ref<any>(null)

onMounted(async () => {
  try {
    console.log('加载编辑器配置，文档 ID:', documentId)
    const response = await onlyofficeApi.getConfig(documentId)
    console.log('后端返回配置:', response)
    console.log( response.onlyoffice_server)
    // 设置服务器 URL
    documentServerUrl.value = response.onlyoffice_server
    
    // 设置编辑器配置
    editorConfig.value = response.config
    
    console.log('编辑器配置已设置:', {
      serverUrl: documentServerUrl.value,
      config: editorConfig.value
    })
    
    loading.value = false
  } catch (err: any) {
    console.error('加载编辑器配置失败:', err)
    error.value = err.response?.data?.error || err.message || '加载编辑器配置失败'
    loading.value = false
  }
})

const onDocumentReady = () => {
  console.log('Document is ready')
  message.success('文档已加载')
}

const onLoadComponentError = (errorCode: number, errorDescription: string) => {
  console.error('OnlyOffice 组件加载错误:', errorCode, errorDescription)
  
  let errorMessage = ''
  switch(errorCode) {
    case -1:
      errorMessage = '未知错误: ' + errorDescription
      break
    case -2:
      errorMessage = `无法从 ${documentServerUrl.value} 加载 DocsAPI: ` + errorDescription
      break
    case -3:
      errorMessage = 'DocsAPI 未定义: ' + errorDescription
      break
    default:
      errorMessage = errorDescription
  }
  
  error.value = errorMessage
  message.error(errorMessage)
}
</script>

<style scoped>
.editor-view {
  width: 100%;
  height: 100vh;
  position: relative;
}

.loading {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
}

.error {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100vh;
}
</style>
