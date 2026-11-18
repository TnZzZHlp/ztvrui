<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { isAxiosError } from 'axios'
import { Loader2 } from 'lucide-vue-next'
import { editProfile } from '@/api/manage/auth'
import { useAuthStore } from '@/stores/auth'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { Input } from '@/components/ui/input'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'

const { t } = useI18n()
const authStore = useAuthStore()

const open = defineModel<boolean>('open', { required: true })
const currentPassword = ref('')
const username = ref('')
const password = ref('')
const confirmPassword = ref('')
const submitting = ref(false)

const resetForm = () => {
  currentPassword.value = ''
  username.value = authStore.username || ''
  password.value = ''
  confirmPassword.value = ''
}

watch(open, (isOpen) => {
  if (isOpen) {
    resetForm()
  }
})

const handleSubmit = async () => {
  if (submitting.value) return
  const name = username.value.trim()

  if (!currentPassword.value || !name || !password.value || !confirmPassword.value) {
    showSnackBar(t('auth.error.emptyFields'), 'error')
    return
  }

  if (password.value !== confirmPassword.value) {
    showSnackBar(t('auth.error.passwordMismatch'), 'error')
    return
  }

  if (password.value.length < 6) {
    showSnackBar(t('auth.error.passwordTooShort'), 'error')
    return
  }

  if (new TextEncoder().encode(password.value).length > 72) {
    showSnackBar(t('auth.error.passwordTooLong'), 'error')
    return
  }

  submitting.value = true
  try {
    await editProfile({
      current_password: currentPassword.value,
      username: name,
      password: password.value,
    })
    showSnackBar(t('common.updateSuccess'), 'success')
    open.value = false
    resetForm()
  } catch (error) {
    if (isAxiosError(error) && error.response?.status === 403) {
      showSnackBar(t('auth.error.wrongCurrentPassword'), 'error')
    } else if (isAxiosError(error) && error.response?.status === 429) {
      showSnackBar(t('network.delete.tooManyAttempts'), 'error')
    } else {
      showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(error)}`, 'error')
    }
  } finally {
    submitting.value = false
  }
}

const handleOpenChange = (newOpen: boolean) => {
  if (submitting.value) return
  open.value = newOpen
  if (!newOpen) {
    resetForm()
  }
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>{{ t('auth.modifyInfo') }}</DialogTitle>
        <DialogDescription>{{ t('auth.pleaseEnterCredentials') }}</DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit">
        <div class="grid gap-4 py-4">
          <div class="flex flex-col gap-2">
            <label for="profile-current-password" class="text-sm font-medium">
              {{ t('auth.currentPassword') }}
            </label>
            <Input
              id="profile-current-password"
              v-model="currentPassword"
              type="password"
              :placeholder="t('auth.currentPassword')"
              :disabled="submitting"
              autocomplete="current-password"
              required
            />
          </div>
          <div class="flex flex-col gap-2">
            <label for="profile-username" class="text-sm font-medium">
              {{ t('auth.username') }}
            </label>
            <Input
              id="profile-username"
              v-model="username"
              type="text"
              :placeholder="t('auth.username')"
              :disabled="submitting"
              autocomplete="username"
              required
            />
          </div>
          <div class="flex flex-col gap-2">
            <label for="profile-new-password" class="text-sm font-medium">
              {{ t('auth.newPassword') }}
            </label>
            <Input
              id="profile-new-password"
              v-model="password"
              type="password"
              :placeholder="t('auth.newPassword')"
              :disabled="submitting"
              autocomplete="new-password"
              required
            />
          </div>
          <div class="flex flex-col gap-2">
            <label for="profile-confirm-password" class="text-sm font-medium">
              {{ t('auth.confirmPassword') }}
            </label>
            <Input
              id="profile-confirm-password"
              v-model="confirmPassword"
              type="password"
              :placeholder="t('auth.confirmPassword')"
              :disabled="submitting"
              autocomplete="new-password"
              required
            />
          </div>
        </div>
        <DialogFooter>
          <Button type="button" variant="outline" :disabled="submitting" @click="handleOpenChange(false)">
            {{ t('common.cancel') }}
          </Button>
          <Button type="submit" :disabled="submitting">
            <Loader2 v-if="submitting" class="h-4 w-4 animate-spin" />
            <span>{{ t('common.confirm') }}</span>
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
