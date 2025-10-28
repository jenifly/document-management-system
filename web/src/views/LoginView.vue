<template>
  <div class="login-view">
    <n-card class="login-card">
      <template #header>
        <div class="header">
          <h2>文档管理系统</h2>
          <p>欢迎登录</p>
        </div>
      </template>

      <n-form
        ref="formRef"
        :model="loginForm"
        :rules="rules"
        label-placement="left"
        label-width="80"
      >
        <n-form-item path="username">
          <n-input
            v-model:value="loginForm.username"
            placeholder="用户名"
            size="large"
          >
            <template #prefix>
              <n-icon><PersonOutline /></n-icon>
            </template>
          </n-input>
        </n-form-item>

        <n-form-item path="password">
          <n-input
            v-model:value="loginForm.password"
            type="password"
            placeholder="密码"
            size="large"
            show-password-on="click"
          >
            <template #prefix>
              <n-icon><LockClosedOutline /></n-icon>
            </template>
          </n-input>
        </n-form-item>

        <n-form-item>
          <n-button
            type="primary"
            size="large"
            :loading="loading"
            @click="handleLogin"
            block
          >
            登录
          </n-button>
        </n-form-item>

        <div class="footer">
          <span>还没有账号？</span>
          <n-button text type="primary" @click="router.push('/register')">
            立即注册
          </n-button>
        </div>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, toRaw } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { NCard, NForm, NFormItem, NInput, NButton, NIcon, useMessage, type FormInst, type FormRules } from 'naive-ui'
import { PersonOutline, LockClosedOutline } from '@vicons/ionicons5'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const route = useRoute()
const userStore = useUserStore()
const message = useMessage()

const formRef = ref<FormInst | null>(null)
const loading = ref(false)

const loginForm = reactive({
  username: '',
  password: ''
})

const rules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 100, message: '用户名长度为 3-100 个字符', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少 6 个字符', trigger: 'blur' }
  ]
}

const handleLogin = async () => {
  if (!formRef.value) return

  try {
    await formRef.value.validate()
    
    loading.value = true
    
    // 获取原始值并创建新的普通对象
    const rawForm = toRaw(loginForm)
    const credentials = {
      username: String(rawForm.username),
      password: String(rawForm.password)
    }
    
    console.log('LoginView - rawForm:', rawForm)
    console.log('LoginView - credentials:', credentials)
    console.log('LoginView - JSON:', JSON.stringify(credentials))
    
    await userStore.login(credentials)
    
    message.success('登录成功')
    
    // 跳转到之前访问的页面或首页
    const redirect = route.query.redirect as string
    router.push(redirect || '/')
  } catch (error: any) {
    console.error('Login error:', error)
    if (error?.response) {
      message.error(error.response?.data?.error || '登录失败')
    } else {
      message.error('登录失败，请检查网络连接')
    }
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.login-view {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.login-card {
  width: 400px;
}

.header {
  text-align: center;
}

.header h2 {
  margin: 0 0 10px 0;
  font-size: 28px;
  color: #303133;
}

.header p {
  margin: 0;
  font-size: 14px;
  color: #909399;
}

.footer {
  text-align: center;
  font-size: 14px;
  color: #606266;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 4px;
}
</style>
