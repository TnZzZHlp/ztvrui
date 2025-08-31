import { check } from '@/api/manage/check'
import { tokenManager } from '@/utils/tokenManager'
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
  // If going to login page and already authenticated, redirect to networks
  if (to.name === 'login') {
    if (tokenManager.getToken() && !tokenManager.isTokenExpired()) {
      return { name: 'networks' }
    }
    return true
  }

  // For protected routes, check authentication
  try {
    // First check local token
    if (!tokenManager.getToken() || tokenManager.isTokenExpired()) {
      throw new Error('No valid token')
    }

    // Verify with server
    await check()
    return true
  } catch (error) {
    console.log('Authentication check failed:', error)
    // Clear any invalid tokens
    tokenManager.clearToken()
    return { name: 'login' }
  }
})

export default router
