import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import api from '@/api'
import type { User, LoginResponse, RegisterRequest } from '@/types'

export const useUserStore = defineStore('user', () => {
  const token = ref<string>(localStorage.getItem('token') || '')
  const user = ref<User | null>(null)

  const isLoggedIn = computed(() => !!token.value)

  async function login(credentials: { username: string; password: string }): Promise<LoginResponse> {
    console.log('Store login called with:', credentials)
    console.log('Type of credentials:', typeof credentials)
    console.log('Credentials keys:', Object.keys(credentials))
    
    const response = await api.auth.login(credentials)
    token.value = response.token
    user.value = response.user
    localStorage.setItem('token', response.token)
    return response
  }

  async function register(userData: RegisterRequest): Promise<User> {
    const response = await api.auth.register(userData)
    return response
  }

  async function getCurrentUser(): Promise<User> {
    try {
      const response = await api.auth.getCurrentUser()
      user.value = response
      return response
    } catch (error) {
      logout()
      throw error
    }
  }

  function logout(): void {
    token.value = ''
    user.value = null
    localStorage.removeItem('token')
  }

  return {
    token,
    user,
    isLoggedIn,
    login,
    register,
    getCurrentUser,
    logout
  }
})

