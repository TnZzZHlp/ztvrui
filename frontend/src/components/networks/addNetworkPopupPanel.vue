<script setup lang="ts">
import { generateRandomNetworkId } from '@/api/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { eventBus } from '@/utils/eventBus'

const { container } = defineProps({
  container: HTMLDivElement,
})

const close = ref(false)
const { t } = useI18n()

const handleClick = (e: Event) => {
  const form = e.target as HTMLFormElement
  const networkName = form.networkName.value.trim()

  if (networkName) {
    generateRandomNetworkId({
      name: networkName,
    })
      .then(() => {
        showSnackBar(t('network.add.success'), 'success')

        //  Dispatch an event to notify that the network list has changed
        eventBus.dispatchEvent(new Event('networkListChanged'))
        closePopupPanel()
      })
      .catch((error) => {
        showSnackBar(t('network.add.error') + `: ${error.message}`, 'error')
      })
  } else {
    showSnackBar(t('network.needName'), 'error')
  }
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
    <div class="bg-white rounded p-5 flex flex-col justify-between">
      <form @submit.prevent="handleClick">
        <input
          type="text"
          :placeholder="t('network.name')"
          class="border p-4 rounded"
          name="networkName"
        />
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
