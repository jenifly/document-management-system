<template>
  <el-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    title="版本历史"
    width="800px"
  >
    <el-table
      v-loading="loading"
      :data="versions"
      style="width: 100%"
    >
      <el-table-column label="版本" width="80">
        <template #default="{ row }">
          <el-tag type="info">v{{ row.version }}</el-tag>
        </template>
      </el-table-column>
      
      <el-table-column label="文件大小" width="120">
        <template #default="{ row }">
          {{ formatFileSize(row.file_size) }}
        </template>
      </el-table-column>
      
      <el-table-column label="备注" prop="comment">
        <template #default="{ row }">
          {{ row.comment || '-' }}
        </template>
      </el-table-column>
      
      <el-table-column label="创建时间" width="180">
        <template #default="{ row }">
          {{ formatDate(row.created_at) }}
        </template>
      </el-table-column>
      
      <el-table-column label="操作" width="150">
        <template #default="{ row }">
          <el-button type="primary" text size="small" @click="handleRestore(row)">
            恢复此版本
          </el-button>
        </template>
      </el-table-column>
    </el-table>

    <el-empty v-if="!loading && versions.length === 0" description="暂无版本历史" />
  </el-dialog>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { ElMessage, ElMessageBox } from 'element-plus'

interface Props {
  modelValue: boolean
  documentId: string
}

interface Version {
  id: string
  document_id: string
  version: number
  file_path: string
  file_size: number
  comment?: string
  created_by: string
  created_at: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const loading = ref(false)
const versions = ref<Version[]>([])

watch(() => props.modelValue, (newVal) => {
  if (newVal && props.documentId) {
    loadVersions()
  }
})

const loadVersions = async () => {
  try {
    loading.value = true
    // TODO: 添加获取文档版本历史的 API
    // versions.value = await versionsApi.list(props.documentId)
    versions.value = []
    ElMessage.info('版本历史功能正在开发中')
  } catch (error: any) {
    ElMessage.error(error.response?.data?.error || '加载版本历史失败')
  } finally {
    loading.value = false
  }
}

const handleRestore = async (version: Version) => {
  try {
    await ElMessageBox.confirm(
      `确定要恢复到版本 v${version.version} 吗？`,
      '提示',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      }
    )

    // TODO: 实现版本恢复功能
    ElMessage.info('版本恢复功能正在开发中')
  } catch (error) {
    // 用户取消
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
/* 样式可以根据需要添加 */
</style>

