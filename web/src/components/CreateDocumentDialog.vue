<template>
  <el-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    title="创建新文档"
    width="600px"
  >
    <el-form :model="form" label-width="100px">
      <el-form-item label="文档类型">
        <el-select v-model="form.docType" placeholder="选择文档类型">
          <el-option label="Word 文档 (.docx)" value="docx" />
          <el-option label="Excel 表格 (.xlsx)" value="xlsx" />
          <el-option label="PowerPoint (.pptx)" value="pptx" />
          <el-option label="文本文件 (.txt)" value="txt" />
        </el-select>
      </el-form-item>

      <el-form-item label="文件名">
        <el-input
          v-model="form.fileName"
          placeholder="请输入文件名（不含扩展名）"
        />
      </el-form-item>

      <el-form-item label="描述">
        <el-input
          v-model="form.description"
          type="textarea"
          :rows="3"
          placeholder="请输入描述（可选）"
        />
      </el-form-item>

      <el-form-item label="标签">
        <el-input
          v-model="form.tags"
          placeholder="标签，用逗号分隔（可选）"
        />
      </el-form-item>
    </el-form>

    <template #footer>
      <el-button @click="$emit('update:modelValue', false)">取消</el-button>
      <el-button type="primary" @click="handleCreate" :loading="loading">
        创建
      </el-button>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch } from 'vue'
import { ElMessage } from 'element-plus'

interface Props {
  modelValue: boolean
  folderId?: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'created'): void
}>()

const loading = ref(false)
const form = reactive({
  docType: 'docx',
  fileName: '',
  description: '',
  tags: ''
})

watch(() => props.modelValue, (newVal) => {
  if (!newVal) {
    // 重置表单
    form.docType = 'docx'
    form.fileName = ''
    form.description = ''
    form.tags = ''
  }
})

const handleCreate = async () => {
  if (!form.fileName) {
    ElMessage.warning('请输入文件名')
    return
  }

  try {
    loading.value = true
    
    // TODO: 实现创建空文档的功能
    // 1. 在后端创建一个空的 Office 文档模板
    // 2. 上传到 MinIO
    // 3. 创建文档记录
    
    ElMessage.info('此功能正在开发中，将在后续版本中提供')
    
    // 临时实现：提示用户上传文件
    emit('update:modelValue', false)
    emit('created')
  } catch (error: any) {
    ElMessage.error(error.response?.data?.error || '创建失败')
  } finally {
    loading.value = false
  }
}

const getFileExtension = (type: string): string => {
  const extensions: Record<string, string> = {
    docx: '.docx',
    xlsx: '.xlsx',
    pptx: '.pptx',
    txt: '.txt'
  }
  return extensions[type] || '.docx'
}
</script>

<style scoped>
/* 样式可以根据需要添加 */
</style>

