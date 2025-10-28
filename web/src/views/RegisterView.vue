<template>
  <div class="register-view">
    <n-card class="register-card">
      <template #header>
        <div class="header">
          <h2>文档管理系统</h2>
          <p>创建新账户</p>
        </div>
      </template>

      <n-form
        ref="formRef"
        :model="registerForm"
        :rules="rules"
        label-placement="left"
        label-width="80"
      >
        <n-form-item path="username">
          <n-input
            v-model:value="registerForm.username"
            placeholder="用户名"
            size="large"
          >
            <template #prefix>
              <n-icon><PersonOutline /></n-icon>
            </template>
          </n-input>
        </n-form-item>

        <n-form-item path="email">
          <n-input
            v-model:value="registerForm.email"
            placeholder="邮箱"
            size="large"
          >
            <template #prefix>
              <n-icon><MailOutline /></n-icon>
            </template>
          </n-input>
        </n-form-item>

        <n-form-item path="full_name">
          <n-input
            v-model:value="registerForm.full_name"
            placeholder="全名（可选）"
            size="large"
          >
            <template #prefix>
              <n-icon><PersonCircleOutline /></n-icon>
            </template>
          </n-input>
        </n-form-item>

        <n-form-item path="password">
          <n-input
            v-model:value="registerForm.password"
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

        <n-form-item path="confirmPassword">
          <n-input
            v-model:value="registerForm.confirmPassword"
            type="password"
            placeholder="确认密码"
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
            @click="handleRegister"
            block
          >
            注册
          </n-button>
        </n-form-item>

        <div class="footer">
          <span>已有账号？</span>
          <n-button text type="primary" @click="router.push('/login')">
            立即登录
          </n-button>
        </div>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useRouter } from 'vue-router'
import { NCard, NForm, NFormItem, NInput, NButton, NIcon, useMessage, type FormInst, type FormRules, type FormItemRule } from 'naive-ui'
import { PersonOutline, PersonCircleOutline, LockClosedOutline, MailOutline } from '@vicons/ionicons5'
import authApi from '@/api/auth'

const router = useRouter()
const message = useMessage()

const formRef = ref<FormInst | null>(null)
const loading = ref(false)

const registerForm = reactive({
  username: '',
  email: '',
  full_name: '',
  password: '',
  confirmPassword: ''
})

const validateConfirmPassword = (rule: FormItemRule, value: string): boolean | Error => {
  if (value === '') {
    return new Error('请再次输入密码')
  } else if (value !== registerForm.password) {
    return new Error('两次输入的密码不一致')
  }
  return true
}

const rules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 100, message: '用户名长度为 3-100 个字符', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱', trigger: 'blur' },
    { type: 'email', message: '请输入有效的邮箱地址', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码至少 6 个字符', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, validator: validateConfirmPassword, trigger: 'blur' }
  ]
}

const handleRegister = async () => {
  if (!formRef.value) return

  try {
    await formRef.value.validate()
    
    loading.value = true
    
    await authApi.register({
      username: registerForm.username,
      email: registerForm.email,
      full_name: registerForm.full_name || undefined,
      password: registerForm.password
    })
    
    message.success('注册成功，请登录')
    router.push('/login')
  } catch (error: any) {
    message.error(error.response?.data?.error || '注册失败')
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.register-view {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.register-card {
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
