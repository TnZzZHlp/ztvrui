<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { Languages, LogOut, Plus, UserCog } from 'lucide-vue-next'
import NetworksComponent from '@/components/Networks/NetworkList.vue'
import AddNetworkDialog from '@/components/Networks/AddNetworkDialog.vue'
import ModifyInfoDialog from '@/components/Networks/ModifyInfoDialog.vue'
import { Button } from '@/components/ui/button'
import { setLocale } from '@/i18n'
import { useAuthStore } from '@/stores/auth'
import { showSnackBar } from '@/utils/showSnackBar'

const { t, locale } = useI18n()
const router = useRouter()
const authStore = useAuthStore()

const showAddDialog = ref(false)
const showModifyDialog = ref(false)

const toggleLocale = () => {
  setLocale(locale.value === 'zh-CN' ? 'en' : 'zh-CN')
}

const handleLogout = async () => {
  authStore.clearAuth()
  showSnackBar(t('auth.logoutSuccess'), 'success')
  await router.push({ name: 'login' })
}
</script>

<template>
  <div class="min-h-screen bg-muted/20">
    <header class="sticky top-0 z-20 border-b bg-background/95 backdrop-blur">
      <div class="mx-auto flex h-[60px] max-w-5xl items-center justify-between gap-2 px-4">
        <div class="flex items-center gap-2.5">
          <img src="/icon.png" alt="ZTVRUI" class="h-7 w-7 rounded" />
          <span class="text-lg font-bold tracking-tight sm:text-xl">{{ t('network.default') }}</span>
        </div>

        <div class="flex flex-wrap items-center gap-2">
          <Button type="button" variant="ghost" size="sm" @click="toggleLocale">
            <Languages class="h-4 w-4" />
            <span class="hidden sm:inline">{{ locale === 'zh-CN' ? 'EN' : '中文' }}</span>
          </Button>
          <Button type="button" variant="outline" size="sm" @click="showModifyDialog = true">
            <UserCog class="h-4 w-4" />
            <span class="hidden sm:inline">{{ t('auth.modifyInfo') }}</span>
          </Button>
          <Button type="button" size="sm" @click="showAddDialog = true">
            <Plus class="h-4 w-4" />
            <span>{{ t('network.add.default') }}</span>
          </Button>
          <Button
            type="button"
            variant="ghost"
            size="sm"
            :aria-label="t('auth.logout')"
            @click="handleLogout"
          >
            <LogOut class="h-4 w-4" />
            <span class="hidden md:inline">{{ t('auth.logout') }}</span>
          </Button>
        </div>
      </div>
    </header>

    <main class="mx-auto w-full max-w-5xl px-4 py-6">
      <NetworksComponent @add="showAddDialog = true" />
    </main>

    <AddNetworkDialog v-model:open="showAddDialog" />
    <ModifyInfoDialog v-model:open="showModifyDialog" />
  </div>
</template>
