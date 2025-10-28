import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import { useUserStore } from '@/stores/user'

const routes: RouteRecordRaw[] = [
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/LoginView.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/register',
    name: 'register',
    component: () => import('@/views/RegisterView.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/',
    component: () => import('@/views/LayoutView.vue'),
    meta: { requiresAuth: true },
    children: [
      {
        path: '',
        name: 'home',
        component: () => import('@/views/HomeView.vue')
      },
      {
        path: 'documents',
        name: 'documents',
        component: () => import('@/views/DocumentsView.vue')
      },
      {
        path: 'documents/:id',
        name: 'document-detail',
        component: () => import('@/views/DocumentDetailView.vue')
      },
      {
        path: 'search',
        name: 'search',
        component: () => import('@/views/SearchView.vue')
      },
      {
        path: 'shared',
        name: 'shared',
        component: () => import('@/views/SharedView.vue')
      },
      {
        path: 'profile',
        name: 'profile',
        component: () => import('@/views/ProfileView.vue')
      }
    ]
  },
  {
    path: '/share/:token',
    name: 'share-access',
    component: () => import('@/views/ShareAccessView.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/editor/:id',
    name: 'editor',
    component: () => import('@/views/EditorView.vue'),
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const userStore = useUserStore()
  const requiresAuth = to.matched.some(record => record.meta.requiresAuth !== false)

  if (requiresAuth && !userStore.isLoggedIn) {
    next({ name: 'login', query: { redirect: to.fullPath } })
  } else if ((to.name === 'login' || to.name === 'register') && userStore.isLoggedIn) {
    next({ name: 'home' })
  } else {
    next()
  }
})

export default router

