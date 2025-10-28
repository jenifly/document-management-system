import request from '@/utils/request'
import type { LoginRequest, LoginResponse, RegisterRequest, User } from '@/types'

export default {
  // 用户登录
  login(credentials: LoginRequest): Promise<LoginResponse> {
    console.log('API login called with:', credentials)
    return request.post('/auth/login', credentials)
  },

  // 用户注册
  register(userData: RegisterRequest): Promise<User> {
    console.log('API register called with:', userData)
    return request.post('/auth/register', userData)
  },

  // 获取当前用户信息
  getCurrentUser(): Promise<User> {
    return request.get('/auth/me')
  }
}

