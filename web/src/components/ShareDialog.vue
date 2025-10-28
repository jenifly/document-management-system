<template>
  <n-modal
    :show="modelValue"
    @update:show="$emit('update:modelValue', $event)"
    preset="card"
    title="分享管理"
    style="width: 700px"
  >
    <n-space vertical :size="20">
      <!-- 创建分享链接表单 -->
      <n-card title="创建分享链接" size="small">
        <n-form :model="createForm" label-placement="left" label-width="100">
          <n-form-item label="权限类型">
            <n-select 
              v-model:value="createForm.permission" 
              placeholder="选择权限类型"
              :options="permissionOptions"
            />
          </n-form-item>
          <n-form-item label="访问密码">
            <n-input
              v-model:value="createForm.password"
              type="password"
              placeholder="留空表示不设置密码"
            />
          </n-form-item>
          <n-form-item label="最大访问数">
            <n-input-number
              v-model:value="createForm.maxAccessCount"
              :min="1"
              :max="1000"
              placeholder="留空表示不限制"
              clearable
            />
          </n-form-item>
          <n-form-item label="过期时间">
            <n-date-picker
              v-model:value="createForm.expiresAt"
              type="datetime"
              placeholder="选择过期时间（可选）"
              clearable
            />
          </n-form-item>
          <n-form-item>
            <n-button type="primary" @click="handleCreateShare">
              创建链接
            </n-button>
          </n-form-item>
        </n-form>
      </n-card>

      <!-- 分享链接列表 -->
      <n-card title="已创建的分享链接" size="small">
        <n-data-table
          :loading="loading"
          :columns="columns"
          :data="shareLinks"
          :pagination="false"
        />

        <n-empty v-if="!loading && shareLinks.length === 0" description="暂无分享链接" />
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
  NSelect,
  NInput,
  NInputNumber,
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
import type { ShareLink, PermissionType } from '@/types'

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
const shareLinks = ref<ShareLink[]>([])

const createForm = reactive({
  permission: 'read' as PermissionType,
  password: '',
  maxAccessCount: undefined as number | undefined,
  expiresAt: null as number | null
})

const permissionOptions = [
  { label: '只读', value: 'read' },
  { label: '可编辑', value: 'write' }
]

const columns: DataTableColumns<ShareLink> = [
  {
    title: '链接',
    key: 'token',
    width: 200,
    render: (row) => {
      const url = getShareUrl(row.token)
      return h(NSpace, { align: 'center' }, {
        default: () => [
          h(NInput, { value: url, readonly: true, size: 'small' }),
          h(NButton, {
            text: true,
            type: 'primary',
            size: 'small',
            onClick: () => copyToClipboard(url)
          }, { default: () => '复制' })
        ]
      })
    }
  },
  {
    title: '权限',
    key: 'permission',
    width: 80,
    render: (row) => h(NTag, { 
      type: row.permission === 'read' ? 'info' : 'success',
      size: 'small'
    }, { default: () => row.permission === 'read' ? '只读' : '可编辑' })
  },
  {
    title: '访问次数',
    key: 'access_count',
    width: 100,
    render: (row) => {
      const text = row.max_access_count 
        ? `${row.access_count} / ${row.max_access_count}`
        : `${row.access_count}`
      return text
    }
  },
  {
    title: '过期时间',
    key: 'expires_at',
    width: 150,
    render: (row) => row.expires_at ? formatDate(row.expires_at) : '永不过期'
  },
  {
    title: '操作',
    key: 'actions',
    width: 80,
    render: (row) => h(
      NButton,
      {
        text: true,
        type: 'error',
        size: 'small',
        onClick: () => handleDeleteShare(row)
      },
      { default: () => '删除' }
    )
  }
]

watch(() => props.modelValue, (newVal) => {
  if (newVal && props.documentId) {
    loadShareLinks()
  }
})

const loadShareLinks = async () => {
  // TODO: 添加后端接口来列出文档的分享链接
  shareLinks.value = []
}

const handleCreateShare = async () => {
  if (!createForm.permission) {
    message.warning('请选择权限类型')
    return
  }

  try {
    const link = await permissionsApi.createShareLink(props.documentId, {
      permission: createForm.permission,
      password: createForm.password || undefined,
      max_access_count: createForm.maxAccessCount,
      expires_at: createForm.expiresAt ? new Date(createForm.expiresAt).toISOString() : undefined
    })

    message.success('分享链接创建成功')
    shareLinks.value.push(link)
    
    // 重置表单
    createForm.permission = 'read'
    createForm.password = ''
    createForm.maxAccessCount = undefined
    createForm.expiresAt = null
  } catch (error: any) {
    message.error(error.response?.data?.error || '创建分享链接失败')
  }
}

const handleDeleteShare = async (link: ShareLink) => {
  dialog.warning({
    title: '提示',
    content: '确定要删除此分享链接吗？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: async () => {
      try {
        await permissionsApi.deleteShareLink(link.id)
        message.success('分享链接已删除')
        shareLinks.value = shareLinks.value.filter(l => l.id !== link.id)
      } catch (error: any) {
        message.error(error.response?.data?.error || '删除分享链接失败')
      }
    }
  })
}

const getShareUrl = (token: string): string => {
  const baseUrl = window.location.origin
  return `${baseUrl}/share/${token}`
}

const copyToClipboard = async (text: string) => {
  try {
    await navigator.clipboard.writeText(text)
    message.success('链接已复制到剪贴板')
  } catch (error) {
    message.error('复制失败，请手动复制')
  }
}

const formatDate = (dateStr: string): string => {
  return new Date(dateStr).toLocaleString('zh-CN')
}
</script>
