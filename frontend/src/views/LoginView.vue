<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import { isAxiosError } from 'axios'
import { Eye, EyeOff, Languages, Loader2 } from 'lucide-vue-next'
import { login } from '@/api/manage/auth'
import { setLocale } from '@/i18n'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import logoIcon from '@/assets/icon.png'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'

const { t, locale } = useI18n()
const router = useRouter()

const username = ref('')
const password = ref('')
const showPassword = ref(false)
const loading = ref(false)
const showForgotDialog = ref(false)

const toggleLocale = () => {
  setLocale(locale.value === 'zh-CN' ? 'en' : 'zh-CN')
}

const handleLogin = async () => {
  const trimmedUsername = username.value.trim()
  if (!trimmedUsername || !password.value) {
    showSnackBar(t('auth.pleaseEnterCredentials'), 'error')
    return
  }

  loading.value = true
  try {
    const response = await login({ username: trimmedUsername, password: password.value })
    showSnackBar(`${t('auth.loginSuccessful')} ${response.username}`, 'success')
    await router.push({ name: 'networks' })
  } catch (error) {
    if (isAxiosError(error) && error.response?.status === 429) {
      showSnackBar(getErrorMessage(error) || t('network.delete.tooManyAttempts'), 'error')
    } else if (isAxiosError(error) && error.response?.status === 401) {
      showSnackBar(t('auth.loginFailed'), 'error')
    } else {
      showSnackBar(`${t('auth.loginFailed')}: ${getErrorMessage(error)}`, 'error')
    }
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="relative flex min-h-full flex-col justify-center bg-muted/30 px-4 py-12 sm:px-6 lg:px-8">
    <div class="absolute top-4 right-4">
      <Button type="button" variant="outline" size="sm" @click="toggleLocale">
        <Languages class="h-4 w-4" />
        <span>{{ locale === 'zh-CN' ? 'EN' : '中文' }}</span>
      </Button>
    </div>

    <div class="mx-auto w-full max-w-sm rounded-xl border bg-card p-6 text-card-foreground shadow-sm sm:p-8">
      <div class="flex flex-col items-center text-center">
        <img class="h-20 w-auto" :src="logoIcon" alt="ZTVRUI Icon" />
        <h1 class="mt-4 text-2xl font-bold tracking-tight">ZTVRUI</h1>
        <p class="mt-1 text-sm text-muted-foreground">{{ t('auth.loginSubtitle') }}</p>
      </div>

      <form class="mt-8 space-y-5" @submit.prevent="handleLogin">
        <div class="space-y-2">
          <label for="username" class="block text-sm font-medium text-foreground">
            {{ t('auth.username') }}
          </label>
          <Input
            id="username"
            v-model="username"
            type="text"
            name="username"
            autocomplete="username"
            :disabled="loading"
            required
          />
        </div>

        <div class="space-y-2">
          <div class="flex items-center justify-between">
            <label for="password" class="block text-sm font-medium text-foreground">
              {{ t('auth.password') }}
            </label>
            <button
              type="button"
              class="text-xs font-medium text-primary hover:underline"
              @click="showForgotDialog = true"
            >
              {{ t('auth.forgotPassword') }}
            </button>
          </div>
          <div class="relative">
            <Input
              id="password"
              v-model="password"
              :type="showPassword ? 'text' : 'password'"
              name="password"
              autocomplete="current-password"
              class="pr-10"
              :disabled="loading"
              required
            />
            <button
              type="button"
              class="absolute inset-y-0 right-0 flex items-center px-3 text-muted-foreground hover:text-foreground"
              :aria-label="showPassword ? 'Hide password' : 'Show password'"
              @click="showPassword = !showPassword"
            >
              <EyeOff v-if="showPassword" class="h-4 w-4" />
              <Eye v-else class="h-4 w-4" />
            </button>
          </div>
        </div>

        <Button type="submit" class="w-full" :disabled="loading">
          <Loader2 v-if="loading" class="h-4 w-4 animate-spin" />
          <span>{{ t('auth.login') }}</span>
        </Button>
      </form>
    </div>

    <Dialog :open="showForgotDialog" @update:open="showForgotDialog = $event">
      <DialogContent class="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{{ t('auth.forgotPassword') }}</DialogTitle>
          <DialogDescription class="pt-2 text-sm leading-relaxed">
            {{ t('auth.forgotPasswordHint') }}
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button type="button" @click="showForgotDialog = false">
            {{ t('common.close') }}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>
