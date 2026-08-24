<script setup lang="ts">
import { onActivated, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { LayoutDashboard, Users, ArrowLeft } from 'lucide-vue-next'
import {
  Sidebar,
  SidebarMenuButton,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'

const { t } = useI18n()
const router = useRouter()
const networkDetailStore = useNetworkDetailStore()

const siderBarItems = [
  {
    label: t('network.detail.overview'),
    name: 'networkOverview',
    icon: LayoutDashboard,
  },
  {
    label: t('network.detail.members'),
    name: 'networkMembers',
    icon: Users,
  },
]

const handleNavigation = (name: string) => {
  router.push({ name })
}

const handleReturn = () => {
  router.push({ name: 'networks' })
}

onMounted(() => {
  const id = router.currentRoute.value.params.networkId as string
  console.log('NetworkDetailView activated with networkId:', id)
  if (id) networkDetailStore.getNetworkOverviewData(id)
})
</script>

<template>
  <div>
    <SidebarProvider>
      <div class="flex h-screen w-full">
        <Sidebar collapsible="icon">
          <div data-slot="sidebar-header" data-sidebar="header" class="flex flex-col gap-2 p-2">
            <div class="flex items-center gap-2 p-2">
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">ZTVRUI</span>
              </div>
            </div>
          </div>
          <div data-slot="sidebar-content" data-sidebar="content"
            class="flex min-h-0 flex-1 flex-col gap-2 overflow-auto group-data-[collapsible=icon]:overflow-hidden">
            <div data-slot="sidebar-group" data-sidebar="group" class="relative flex w-full min-w-0 flex-col p-2">
              <div data-slot="sidebar-group-content" data-sidebar="group-content" class="w-full text-sm">
                <ul data-slot="sidebar-menu" data-sidebar="menu" class="flex w-full min-w-0 flex-col gap-1">
                  <li data-slot="sidebar-menu-item" data-sidebar="menu-item" class="group/menu-item relative"
                    v-for="item in siderBarItems" :key="item.name">
                    <SidebarMenuButton :is-active="router.currentRoute.value.name === item.name"
                      @click="handleNavigation(item.name)">
                      <component :is="item.icon" />
                      <span>{{ item.label }}</span>
                    </SidebarMenuButton>
                  </li>
                  <li data-slot="sidebar-menu-item" data-sidebar="menu-item" class="group/menu-item relative">
                    <SidebarMenuButton @click="handleReturn">
                      <ArrowLeft />
                      <span>{{ t('common.return') }}</span>
                    </SidebarMenuButton>
                  </li>
                </ul>
              </div>
            </div>
          </div>
        </Sidebar>

        <main data-slot="sidebar-inset" class="bg-background relative flex w-full flex-1 flex-col">
          <!-- Header -->
          <header class="flex h-[60px] shrink-0 items-center gap-2 border-b px-4">
            <SidebarTrigger />
            <span class="font-bold text-xl">{{ t('network.detail.default') }}</span>
          </header>

          <!-- Main Content -->
          <main class="flex-1 overflow-auto p-1">
            <router-view v-slot="{ Component }">
              <transition name="fade" mode="out-in">
                <component :is="Component" />
              </transition>
            </router-view>
          </main>
        </main>
      </div>
    </SidebarProvider>
  </div>
</template>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
}
</style>
