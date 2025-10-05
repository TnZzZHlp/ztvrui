import { check } from '@/api/manage/check'
import { useAuthStore } from '@/stores/auth'
import { createRouter, createWebHistory } from 'vue-router'
import LoginView from '@/views/LoginView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'login',
      component: LoginView,
    },
    {
      path: '/networks',
      name: 'networks',
      component: () => import('@/views/NetworksView.vue'),
    },
    {
      path: '/network/:networkId',
      name: 'networkDetail',
      component: () => import('@/views/NetworkDetailView.vue'),
      children: [
        {
          path: 'overview',
          name: 'networkOverview',
          component: () => import('@/components/networkDetail/networkOverview.vue'),
        },
        {
          path: 'members',
          name: 'networkMembers',
          component: () => import('@/components/networkDetail/networkMembers.vue'),
        },
      ],
    },
  ],
})

router.beforeEach(async (to) => {
  const authStore = useAuthStore()

  // If going to login page and already authenticated, redirect to networks
  if (to.name === 'login') {
    if (authStore.isAuthenticated) {
      return { name: 'networks' }
    }
    return true
  }

  // For protected routes, check authentication
  try {
    // First check local token
    if (!authStore.token || authStore.isTokenExpired) {
      throw new Error('No valid token')
    }

    // Verify with server
    await check()
    return true
  } catch (error) {
    console.log('Authentication check failed:', error)
    // Clear any invalid tokens
    authStore.clearAuth()
    return { name: 'login' }
  }
})

export default router
