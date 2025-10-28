import axios from 'axios'
import type { AxiosInstance, AxiosResponse } from 'axios'
import { useUserStore } from '@/stores/user'

// 创建 axios 实例
const request: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || '/api',
  timeout: 30000,
  headers: {
    'Content-Type': 'application/json'
  }
})

// 请求拦截器
request.interceptors.request.use(
  (config) => {
    // 确保 Content-Type 被设置
    if (!config.headers['Content-Type']) {
      config.headers['Content-Type'] = 'application/json'
    }
    
    // 添加认证令牌
    const userStore = useUserStore()
    if (userStore.token) {
      config.headers.Authorization = `Bearer ${userStore.token}`
    }
    
    return config
  },
  (error) => {
    console.error('请求错误:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
request.interceptors.response.use(
  (response: AxiosResponse) => {
    return response.data
  },
  (error) => {
    if (error.response) {
      const { status, data } = error.response

      switch (status) {
        case 401:
          console.error('未授权，请重新登录')
          const userStore = useUserStore()
          userStore.logout()
          window.location.href = '/login'
          break
        case 403:
          console.error(data?.error || '权限不足')
          break
        case 404:
          console.error(data?.error || '资源不存在')
          break
        case 500:
          console.error(data?.error || '服务器错误')
          break
        default:
          console.error(data?.error || '请求失败')
      }
    } else if (error.request) {
      console.error('网络错误，请检查网络连接')
    } else {
      console.error('请求配置错误')
    }

    return Promise.reject(error)
  }
)

export default request

