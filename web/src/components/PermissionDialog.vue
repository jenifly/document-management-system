<template>
  <n-modal
    :show="modelValue"
    @update:show="$emit('update:modelValue', $event)"
    preset="card"
    title="权限管理"
    style="width: 800px"
  >
    <n-space vertical :size="20">
      <!-- 添加权限表单 -->
      <n-card title="授予权限" size="small">
        <n-form :model="addForm" label-placement="left" label-width="100">
          <n-form-item label="用户 ID">
            <n-input
              v-model:value="addForm.userId"
              placeholder="输入用户 UUID"
            />
          </n-form-item>
          <n-form-item label="权限类型">
            <n-select 
              v-model:value="addForm.permission" 
              placeholder="选择权限类型"
              :options="permissionOptions"
            />
          </n-form-item>
          <n-form-item label="过期时间">
            <n-date-picker
              v-model:value="addForm.expiresAt"
              type="datetime"
              placeholder="选择过期时间（可选）"
              clearable
            />
          </n-form-item>
          <n-form-item>
            <n-button type="primary" @click="handleGrantPermission">
              授予权限
            </n-button>
          </n-form-item>
        </n-form>
      </n-card>

      <!-- 权限列表 -->
      <n-card title="已授权用户" size="small">
        <n-data-table
          :loading="loading"
          :columns="columns"
          :data="permissions"
          :pagination="false"
        />

        <n-empty v-if="!loading && permissions.length === 0" description="暂无权限记录" />
      </n-card>
    </n-space>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, reactive, watch, h } from 'vue'
import { 
  NModal,
  NCard,
  NSpace,
  NForm,
  NFormItem,
  NInput,
  NSelect,
  NDatePicker,
  NButton,
  NDataTable,
  NEmpty,
  NTag,
  useMessage,
  useDialog,
  type DataTableColumns
} from 'naive-ui'
import permissionsApi from '@/api/permissions'
import type { DocumentPermission, PermissionType } from '@/types'

interface Props {
  modelValue: boolean
  documentId: string
}

const props = defineProps<Props>()
const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const message = useMessage()
const dialog = useDialog()

const loading = ref(false)
const permissions = ref<DocumentPermission[]>([])

const addForm = reactive({
  userId: '',
  permission: '' as PermissionType | '',
  expiresAt: null as number | null
})

const permissionOptions = [
  { label: '读取', value: 'read' },
  { label: '写入', value: 'write' },
  { label: '删除', value: 'delete' },
  { label: '分享', value: 'share' },
  { label: '管理', value: 'admin' }
]

const columns: DataTableColumns<DocumentPermission> = [
  { title: '用户 ID', key: 'user_id', width: 280 },
  {
    title: '权限',
    key: 'permission',
    width: 100,
    render: (row) => h(NTag, { type: getPermissionTagType(row.permission) }, 
      { default: () => getPermissionLabel(row.permission) }
    )
  },
  {
    title: '授予时间',
    key: 'granted_at',
    width: 180,
    render: (row) => formatDate(row.granted_at)
  },
  {
    title: '过期时间',
    key: 'expires_at',
    width: 180,
    render: (row) => row.expires_at ? formatDate(row.expires_at) : '永不过期'
  },
  {
    title: '操作',
    key: 'actions',
    width: 100,
    render: (row) => h(
      NButton,
      {
        text: true,
        type: 'error',
        size: 'small',
        onClick: () => handleRevokePermission(row)
      },
      { default: () => '撤销' }
    )
  }
]

watch(() => props.modelValue, (newVal) => {
  if (newVal && props.documentId) {
    loadPermissions()
  }
})

const loadPermissions = async () => {
  try {
    loading.value = true
    permissions.value = await permissionsApi.list(props.documentId)
  } catch (error: any) {
    message.error(error.response?.data?.error || '加载权限列表失败')
  } finally {
    loading.value = false
  }
}

const handleGrantPermission = async () => {
  if (!addForm.userId || !addForm.permission) {
    message.warning('请填写必填项')
    return
  }

  try {
    await permissionsApi.grant(props.documentId, {
      user_id: addForm.userId,
      permission: addForm.permission as PermissionType,
      expires_at: addForm.expiresAt ? new Date(addForm.expiresAt).toISOString() : undefined
    })

    message.success('权限授予成功')
    addForm.userId = ''
    addForm.permission = ''
    addForm.expiresAt = null
    await loadPermissions()
  } catch (error: any) {
    message.error(error.response?.data?.error || '授予权限失败')
  }
}

const handleRevokePermission = async (permission: DocumentPermission) => {
  dialog.warning({
    title: '提示',
    content: '确定要撤销此权限吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await permissionsApi.revoke(
          props.documentId,
          permission.user_id,
          permission.permission
        )

        message.success('权限已撤销')
        await loadPermissions()
      } catch (error: any) {
        message.error(error.response?.data?.error || '撤销权限失败')
      }
    }
  })
}

const getPermissionLabel = (permission: PermissionType): string => {
  const labels: Record<PermissionType, string> = {
    read: '读取',
    write: '写入',
    delete: '删除',
    share: '分享',
    admin: '管理'
  }
  return labels[permission] || permission
}

const getPermissionTagType = (permission: PermissionType): 'info' | 'success' | 'error' | 'warning' | 'default' => {
  const types: Record<PermissionType, any> = {
    read: 'info',
    write: 'success',
    delete: 'error',
    share: 'warning',
    admin: 'default'
  }
  return types[permission] || 'info'
}

const formatDate = (dateStr: string): string => {
  return new Date(dateStr).toLocaleString('zh-CN')
}
</script>
