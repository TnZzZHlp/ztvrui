<script setup lang="ts">
import { computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRoute, useRouter } from 'vue-router'
import {
  ArrowLeft,
  Copy,
  Languages,
  LayoutDashboard,
  Loader2,
  LogOut,
  RefreshCw,
  Users,
} from 'lucide-vue-next'
import {
  Sidebar,
  SidebarMenuButton,
  SidebarProvider,
  SidebarTrigger,
} from '@/components/ui/sidebar'
import { Button } from '@/components/ui/button'
import { setLocale } from '@/i18n'
import { useAuthStore } from '@/stores/auth'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { copyToClipboard } from '@/utils/copyToClipboard'
import { showSnackBar } from '@/utils/showSnackBar'

const { t, locale } = useI18n()
const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()
const networkDetailStore = useNetworkDetailStore()

const networkId = computed(() => route.params.networkId as string)
const currentNetwork = computed(() =>
  networkDetailStore.networksData.find((item) => item.id === networkId.value),
)

const sidebarItems = computed(() => [
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
])

const handleNavigation = (name: string) => {
  void router.push({ name, params: { networkId: networkId.value } })
}

const handleReturn = () => {
  void router.push({ name: 'networks' })
}

const handleRefresh = () => {
  if (networkId.value) {
    void networkDetailStore.refreshNetworkData(networkId.value)
  }
}

const toggleLocale = () => {
  setLocale(locale.value === 'zh-CN' ? 'en' : 'zh-CN')
}

const handleLogout = async () => {
  authStore.clearAuth()
  showSnackBar(t('auth.logoutSuccess'), 'success')
  await router.push({ name: 'login' })
}

watch(
  networkId,
  (id) => {
    if (id) {
      networkDetailStore.getNetworkOverviewData(id)
    }
  },
  { immediate: true },
)
</script>

<template>
  <div>
    <SidebarProvider>
      <div class="flex h-screen w-full">
        <Sidebar collapsible="icon">
          <div data-slot="sidebar-header" data-sidebar="header" class="flex flex-col gap-2 p-2">
            <div class="flex items-center gap-2 p-2">
              <img src="/icon.png" alt="ZTVRUI" class="h-6 w-6 shrink-0 rounded" />
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate font-semibold">ZTVRUI</span>
              </div>
            </div>
          </div>

          <div
            data-slot="sidebar-content"
            data-sidebar="content"
            class="flex min-h-0 flex-1 flex-col justify-between overflow-auto group-data-[collapsible=icon]:overflow-hidden"
          >
            <div data-slot="sidebar-group" data-sidebar="group" class="relative flex w-full min-w-0 flex-col p-2">
              <div data-slot="sidebar-group-content" data-sidebar="group-content" class="w-full text-sm">
                <ul data-slot="sidebar-menu" data-sidebar="menu" class="flex w-full min-w-0 flex-col gap-1">
                  <li
                    v-for="item in sidebarItems"
                    :key="item.name"
                    data-slot="sidebar-menu-item"
                    data-sidebar="menu-item"
                    class="group/menu-item relative"
                  >
                    <SidebarMenuButton
                      :is-active="route.name === item.name"
                      @click="handleNavigation(item.name)"
                    >
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

            <div class="border-t p-2">
              <ul class="flex w-full min-w-0 flex-col gap-1">
                <li>
                  <SidebarMenuButton @click="toggleLocale">
                    <Languages />
                    <span>{{ locale === 'zh-CN' ? 'English' : '简体中文' }}</span>
                  </SidebarMenuButton>
                </li>
                <li>
                  <SidebarMenuButton @click="handleLogout">
                    <LogOut />
                    <span>{{ t('auth.logout') }}</span>
                  </SidebarMenuButton>
                </li>
              </ul>
            </div>
          </div>
        </Sidebar>

        <main data-slot="sidebar-inset" class="relative flex w-full flex-1 flex-col bg-muted/20">
          <header class="flex h-[60px] shrink-0 items-center justify-between gap-2 border-b bg-background px-4">
            <div class="flex min-w-0 items-center gap-2">
              <SidebarTrigger />
              <span class="truncate text-lg font-bold sm:text-xl">
                {{ currentNetwork?.name || t('network.detail.default') }}
              </span>
              <Button
                v-if="networkId"
                type="button"
                variant="secondary"
                size="sm"
                class="hidden font-mono text-xs sm:inline-flex"
                @click="copyToClipboard(networkId)"
              >
                <span>{{ networkId }}</span>
                <Copy class="h-3 w-3" />
              </Button>
            </div>

            <Button
              type="button"
              variant="outline"
              size="sm"
              :disabled="networkDetailStore.loading"
              @click="handleRefresh"
            >
              <RefreshCw class="h-4 w-4" :class="{ 'animate-spin': networkDetailStore.loading }" />
              <span class="hidden sm:inline">{{ t('common.refresh') }}</span>
            </Button>
          </header>

          <div class="flex-1 overflow-auto p-2 sm:p-4">
            <div
              v-if="networkDetailStore.loading && !currentNetwork"
              class="flex h-48 items-center justify-center gap-2 text-muted-foreground"
            >
              <Loader2 class="h-5 w-5 animate-spin" />
              <span>{{ t('common.loading') }}</span>
            </div>
            <div
              v-else-if="networkDetailStore.error && !currentNetwork"
              class="flex h-48 flex-col items-center justify-center gap-3 text-center"
            >
              <p class="text-sm text-destructive">{{ t('network.detail.failGet') }}: {{ networkDetailStore.error }}</p>
              <Button type="button" variant="outline" size="sm" @click="handleRefresh">
                <RefreshCw class="h-4 w-4" />
                <span>{{ t('common.refresh') }}</span>
              </Button>
            </div>
            <router-view v-else v-slot="{ Component }">
              <transition name="fade" mode="out-in">
                <component :is="Component" />
              </transition>
            </router-view>
          </div>
        </main>
      </div>
    </SidebarProvider>
  </div>
</template>
