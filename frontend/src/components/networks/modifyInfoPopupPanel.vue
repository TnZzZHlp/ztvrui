<script setup lang="ts">
import { editProfile } from '@/api/manage/editProfile'
import { generateRandomNetworkId } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { container } = defineProps({
  container: HTMLDivElement,
})

const close = ref(false)
const { t } = useI18n()

const handleClick = (e: Event) => {
  const form = e.target as HTMLFormElement
  const username = form.username.value.trim()
  const password = form.password.value.trim()

  if (!username || !password) {
    showSnackBar(t('auth.error.emptyFields'), 'error')
    return
  }

  editProfile({ username, password })
    .then(() => {
      showSnackBar(t('common.updateSuccess'), 'success')
      closePopupPanel()
    })
    .catch((error) => {
      showSnackBar(t('common.updateFailed') + error, 'error')
    })
}

const closePopupPanel = () => {
  close.value = true
  setTimeout(() => {
    container?.remove()
  }, 300)
}
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 popup-panel"
    :class="{ 'popup-panel-leave-active': close }"
  >
    <div class="bg-white rounded p-5 flex flex-col justify-between min-w-80">
      <form @submit.prevent="handleClick" class="flex flex-col gap-5">
        <div class="flex flex-col">
          <span>{{ t('auth.username') }}</span>
          <input
            type="text"
            :placeholder="t('auth.username')"
            class="border p-4 rounded"
            name="username"
            autocomplete="off"
          />
        </div>
        <div class="flex flex-col">
          <span>{{ t('auth.password') }}</span>
          <input
            type="password"
            :placeholder="t('auth.password')"
            class="border p-4 rounded"
            name="password"
            autocomplete="off"
          />
        </div>
        <div class="flex justify-between mt-4">
          <button
            class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300"
            @click="closePopupPanel"
            type="button"
          >
            <b>{{ t('common.cancel') }}</b>
          </button>
          <button
            class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300"
            @click="handleClick"
            type="submit"
          >
            <b>{{ t('common.confirm') }}</b>
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
.popup-panel {
  animation: fade-in 0.3s forwards;
}

.popup-panel-leave-active {
  animation: fade-out 0.3s forwards;
}

@keyframes fade-out {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

@keyframes fade-in {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}
</style>
