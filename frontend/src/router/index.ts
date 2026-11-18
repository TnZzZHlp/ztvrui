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
      component: () => import('@/views/NetworkDetailView.vue'),
      children: [
        {
          path: '',
          redirect: (to) => ({
            name: 'networkOverview',
            params: { networkId: to.params.networkId },
          }),
        },
        {
          path: 'overview',
          name: 'networkOverview',
          component: () => import('@/components/NetworkDetail/NetworkOverview.vue'),
        },
        {
          path: 'members',
          name: 'networkMembers',
          component: () => import('@/components/NetworkDetail/NetworkMembers.vue'),
        },
      ],
    },
    {
      path: '/:pathMatch(.*)*',
      redirect: '/networks',
    },
  ],
})

router.beforeEach((to) => {
  const authStore = useAuthStore()

  if (to.name === 'login' && authStore.isAuthenticated) {
    return { name: 'networks' }
  }

  if (to.name !== 'login' && !authStore.isAuthenticated) {
    return { name: 'login' }
  }

  if (authStore.isAuthenticated && authStore.shouldRefreshToken) {
    authStore.refresh().catch(() => {})
  }
})

export default router
