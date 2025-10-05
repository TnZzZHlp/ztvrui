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

router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore()

  // If going to login page and already authenticated, redirect to networks
  if (to.name === 'login' && authStore.isAuthenticated) {
    next({ name: 'networks' })
  } else if (to.name !== 'login' && !authStore.isAuthenticated) {
    next({ name: 'login' })
  } else {
    next()
  }
  console.debug('Navigation to:', to.fullPath)
})

export default router
