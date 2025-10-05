<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { editProfile } from '@/api/manage/editProfile'
import { showSnackBar } from '@/utils/showSnackBar'
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

const open = defineModel<boolean>('open', { required: true })
const username = ref('')
const password = ref('')

const handleSubmit = () => {
  const name = username.value.trim()
  const pass = password.value.trim()

  if (!name || !pass) {
    showSnackBar(t('auth.error.emptyFields'), 'error')
    return
  }

  editProfile({ username: name, password: pass })
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
      open.value = false
      username.value = ''
      password.value = ''
    })
    .catch((error) => {
      showSnackBar(t('common.updateFailed') + error, 'error')
    })
}

const handleOpenChange = (newOpen: boolean) => {
  open.value = newOpen
  if (!newOpen) {
    username.value = ''
    password.value = ''
  }
}
</script>

<template>
  <Dialog :open="open" @update:open="handleOpenChange">
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>{{ t('auth.modifyInfo') }}</DialogTitle>
        <DialogDescription>
          {{ t('auth.modifyInfo') }}
        </DialogDescription>
      </DialogHeader>
      <form @submit.prevent="handleSubmit">
        <div class="grid gap-4 py-4">
          <div class="flex flex-col gap-2">
            <label for="username" class="text-sm font-medium">
              {{ t('auth.username') }}
            </label>
            <input id="username" v-model="username" type="text" :placeholder="t('auth.username')"
              class="border p-3 rounded" autocomplete="off" />
          </div>
          <div class="flex flex-col gap-2">
            <label for="password" class="text-sm font-medium">
              {{ t('auth.password') }}
            </label>
            <input id="password" v-model="password" type="password" :placeholder="t('auth.password')"
              class="border p-3 rounded" autocomplete="off" />
          </div>
        </div>
        <DialogFooter>
          <Button type="button" variant="outline" @click="open = false">
            {{ t('common.cancel') }}
          </Button>
          <Button type="submit">
            {{ t('common.confirm') }}
          </Button>
        </DialogFooter>
      </form>
    </DialogContent>
  </Dialog>
</template>
