<template>
  <n-layout has-sider class="layout-container">
    <n-layout-sider
      bordered
      show-trigger
      collapse-mode="width"
      :collapsed-width="64"
      :width="200"
      :native-scrollbar="false"
      class="sidebar"
    >
      <div class="logo">
        <h2>文档管理</h2>
      </div>
      <n-menu
        :value="currentRoute"
        :options="menuOptions"
        @update:value="handleMenuSelect"
      />
    </n-layout-sider>

    <n-layout>
      <n-layout-header bordered class="header">
        <div class="header-content">
          <n-breadcrumb>
            <n-breadcrumb-item @click="router.push('/')">
              首页
            </n-breadcrumb-item>
            <n-breadcrumb-item v-if="currentRouteName">
              {{ currentRouteName }}
            </n-breadcrumb-item>
          </n-breadcrumb>

          <div class="user-section">
            <n-dropdown :options="userDropdownOptions" @select="handleUserCommand">
              <n-button text>
                <n-icon size="18"><PersonOutline /></n-icon>
                {{ userStore.user?.username || '用户' }}
                <n-icon size="14"><ChevronDownOutline /></n-icon>
              </n-button>
            </n-dropdown>
          </div>
        </div>
      </n-layout-header>

      <n-layout-content class="main-content" content-style="padding: 24px;">
        <router-view />
      </n-layout-content>
    </n-layout>
  </n-layout>
</template>

<script setup lang="ts">
import { computed, h } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { 
  NLayout,
  NLayoutSider,
  NLayoutHeader,
  NLayoutContent,
  NMenu,
  NBreadcrumb,
  NBreadcrumbItem,
  NButton,
  NDropdown,
  NIcon,
  useDialog,
  useMessage,
  type MenuOption
} from 'naive-ui'
import {
  HomeOutline,
  FolderOpenOutline,
  SearchOutline,
  ShareSocialOutline,
  PersonOutline,
  ChevronDownOutline,
  LogOutOutline
} from '@vicons/ionicons5'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()
const dialog = useDialog()
const message = useMessage()

const currentRoute = computed(() => route.path)
const currentRouteName = computed(() => {
  const nameMap: Record<string, string> = {
    '/': '首页',
    '/documents': '我的文档',
    '/search': '搜索',
    '/shared': '我的分享',
    '/profile': '个人信息'
  }
  return nameMap[route.path] || route.meta?.title as string
})

const renderIcon = (icon: any) => {
  return () => h(NIcon, null, { default: () => h(icon) })
}

const menuOptions: MenuOption[] = [
  {
    label: '首页',
    key: '/',
    icon: renderIcon(HomeOutline)
  },
  {
    label: '我的文档',
    key: '/documents',
    icon: renderIcon(FolderOpenOutline)
  },
  {
    label: '搜索',
    key: '/search',
    icon: renderIcon(SearchOutline)
  },
  {
    label: '我的分享',
    key: '/shared',
    icon: renderIcon(ShareSocialOutline)
  }
]

const userDropdownOptions = [
  {
    label: '个人信息',
    key: 'profile',
    icon: renderIcon(PersonOutline)
  },
  {
    type: 'divider',
    key: 'd1'
  },
  {
    label: '退出登录',
    key: 'logout',
    icon: renderIcon(LogOutOutline)
  }
]

const handleMenuSelect = (key: string) => {
  router.push(key)
}

const handleUserCommand = async (key: string) => {
  switch (key) {
    case 'profile':
      router.push('/profile')
      break
    case 'logout':
      dialog.warning({
        title: '提示',
        content: '确定要退出登录吗？',
        positiveText: '确定',
        negativeText: '取消',
        onPositiveClick: () => {
          userStore.logout()
          message.success('已退出登录')
          router.push('/login')
        }
      })
      break
  }
}
</script>

<style scoped>
.layout-container {
  height: 100vh;
}

.sidebar {
  display: flex;
  flex-direction: column;
}

.logo {
  height: 64px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid var(--n-border-color);
}

.logo h2 {
  margin: 0;
  font-size: 18px;
  color: var(--n-text-color);
}

.header {
  display: flex;
  align-items: center;
  padding: 0 24px;
  height: 64px;
}

.header-content {
  width: 100%;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.user-section {
  display: flex;
  align-items: center;
  gap: 12px;
}

.main-content {
  background: var(--n-color);
  overflow-y: auto;
}
</style>
