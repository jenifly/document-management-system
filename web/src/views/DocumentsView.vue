<template>
  <div class="documents-view">
    <n-card>
      <template #header>
        <div class="header">
          <n-breadcrumb>
            <n-breadcrumb-item>我的文档</n-breadcrumb-item>
          </n-breadcrumb>
          
          <n-space>
            <n-button type="primary" @click="showCreateFolder = true">
              <template #icon>
                <n-icon><FolderOpenOutline /></n-icon>
              </template>
              新建文件夹
            </n-button>
            <n-button type="success" @click="showUploadDialog = true">
              <template #icon>
                <n-icon><CloudUploadOutline /></n-icon>
              </template>
              上传文件
            </n-button>
          </n-space>
        </div>
      </template>
      
      <n-data-table
        :loading="documentStore.loading"
        :columns="columns"
        :data="documentStore.documents"
        :pagination="false"
      />
    </n-card>
    
    <!-- 创建文件夹对话框 -->
    <n-modal v-model:show="showCreateFolder" preset="dialog" title="创建文件夹">
      <n-form :model="folderForm" label-placement="left" label-width="80">
        <n-form-item label="文件夹名">
          <n-input v-model:value="folderForm.name" placeholder="请输入文件夹名" />
        </n-form-item>
        <n-form-item label="描述">
          <n-input
            v-model:value="folderForm.description"
            type="textarea"
            :rows="3"
            placeholder="请输入描述（可选）"
          />
        </n-form-item>
      </n-form>
      <template #action>
        <n-space>
          <n-button @click="showCreateFolder = false">取消</n-button>
          <n-button type="primary" @click="handleCreateFolder">确定</n-button>
        </n-space>
      </template>
    </n-modal>
    
    <!-- 上传文件对话框 -->
    <n-modal v-model:show="showUploadDialog" preset="dialog" title="上传文件">
      <n-upload
        :default-upload="false"
        :max="1"
        @change="handleFileChange"
      >
        <n-upload-dragger>
          <div style="margin-bottom: 12px">
            <n-icon size="48" :depth="3">
              <CloudUploadOutline />
            </n-icon>
          </div>
          <n-text style="font-size: 16px">
            点击或者拖动文件到该区域来上传
          </n-text>
        </n-upload-dragger>
      </n-upload>
      <n-form :model="uploadForm" label-placement="left" label-width="80" style="margin-top: 20px">
        <n-form-item label="描述">
          <n-input
            v-model:value="uploadForm.description"
            type="textarea"
            :rows="2"
            placeholder="请输入描述（可选）"
          />
        </n-form-item>
        <n-form-item label="标签">
          <n-input v-model:value="uploadForm.tags" placeholder="标签，用逗号分隔（可选）" />
        </n-form-item>
      </n-form>
      <template #action>
        <n-space>
          <n-button @click="showUploadDialog = false">取消</n-button>
          <n-button type="primary" @click="handleUpload">上传</n-button>
        </n-space>
      </template>
    </n-modal>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, h } from 'vue'
import { useRouter } from 'vue-router'
import { 
  NCard,
  NBreadcrumb,
  NBreadcrumbItem,
  NSpace,
  NButton,
  NDataTable,
  NModal,
  NForm,
  NFormItem,
  NInput,
  NUpload,
  NUploadDragger,
  NText,
  NIcon,
  useMessage,
  useDialog,
  type DataTableColumns
} from 'naive-ui'
import { FolderOpenOutline, DocumentTextOutline, CloudUploadOutline } from '@vicons/ionicons5'
import { useDocumentStore } from '@/stores/document'
import type { Document as DocumentType } from '@/types'

const router = useRouter()
const documentStore = useDocumentStore()
const message = useMessage()
const dialog = useDialog()

const showCreateFolder = ref(false)
const showUploadDialog = ref(false)
const selectedFile = ref<File | null>(null)

const folderForm = reactive({
  name: '',
  description: ''
})

const uploadForm = reactive({
  description: '',
  tags: ''
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
          onClick: () => handleItemClick(row)
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
  },
  {
    title: '操作',
    key: 'actions',
    width: 250,
    render: (row) => {
      return h('div', { style: { display: 'flex', gap: '8px' } }, [
        !row.is_folder && h(
          NButton,
          {
            text: true,
            type: 'primary',
            onClick: () => handleDownload(row.id)
          },
          { default: () => '下载' }
        ),
        h(
          NButton,
          {
            text: true,
            type: 'warning',
            onClick: () => handleEdit(row)
          },
          { default: () => '编辑' }
        ),
        h(
          NButton,
          {
            text: true,
            type: 'error',
            onClick: () => handleDelete(row)
          },
          { default: () => '删除' }
        )
      ])
    }
  }
]

onMounted(async () => {
  await documentStore.fetchDocuments()
})

const handleItemClick = (item: DocumentType) => {
  if (item.is_folder) {
    documentStore.fetchDocuments(item.id)
  } else {
    router.push(`/documents/${item.id}`)
  }
}

const handleCreateFolder = async () => {
  if (!folderForm.name) {
    message.warning('请输入文件夹名')
    return
  }
  
  try {
    await documentStore.createFolder(folderForm)
    message.success('创建成功')
    showCreateFolder.value = false
    folderForm.name = ''
    folderForm.description = ''
  } catch (error) {
    message.error('创建失败')
  }
}

const handleFileChange = (options: { fileList: any[] }) => {
  if (options.fileList.length > 0) {
    selectedFile.value = options.fileList[0].file
  }
}

const handleUpload = async () => {
  if (!selectedFile.value) {
    message.warning('请选择文件')
    return
  }
  
  const formData = new FormData()
  formData.append('file', selectedFile.value)
  if (uploadForm.description) {
    formData.append('description', uploadForm.description)
  }
  if (uploadForm.tags) {
    formData.append('tags', uploadForm.tags)
  }
  
  try {
    await documentStore.uploadFile(formData)
    message.success('上传成功')
    showUploadDialog.value = false
    uploadForm.description = ''
    uploadForm.tags = ''
    selectedFile.value = null
  } catch (error) {
    message.error('上传失败')
  }
}

const handleDownload = async (id: string) => {
  const url = await documentStore.downloadDocument(id)
  window.open(url, '_blank')
}

const handleEdit = (item: DocumentType) => {
  router.push(`/documents/${item.id}`)
}

const handleDelete = (item: DocumentType) => {
  dialog.warning({
    title: '提示',
    content: `确定要删除 "${item.name}" 吗？`,
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      await documentStore.deleteDocument(item.id)
      message.success('删除成功')
    }
  })
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
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
</style>
