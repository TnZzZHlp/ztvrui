<script setup lang="ts">
import { onActivated, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { LayoutDashboard, Users, ArrowLeft } from 'lucide-vue-next'
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarProvider,
  SidebarInset,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { useNetworkDetailStore } from '@/stores/networkDetail'

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
          <SidebarHeader>
            <div class="flex items-center gap-2 p-2">
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">ZTVRUI</span>
              </div>
            </div>
          </SidebarHeader>
          <SidebarContent>
            <SidebarGroup>
              <SidebarGroupContent>
                <SidebarMenu>
                  <SidebarMenuItem v-for="item in siderBarItems" :key="item.name">
                    <SidebarMenuButton :is-active="router.currentRoute.value.name === item.name"
                      @click="handleNavigation(item.name)">
                      <component :is="item.icon" />
                      <span>{{ item.label }}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                  <SidebarMenuItem>
                    <SidebarMenuButton @click="handleReturn">
                      <ArrowLeft />
                      <span>{{ t('common.return') }}</span>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </SidebarContent>
        </Sidebar>

        <SidebarInset class="flex flex-col">
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
        </SidebarInset>
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
