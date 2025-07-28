<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { onClick, message, type, container } = defineProps({
  onClick: Function,
  message: String,
  type: String,
  container: HTMLDivElement,
})

const close = ref(false)
const { t } = useI18n()

const handleClick = () => {
  if (onClick) {
    onClick()
  }
  closePopupPanel()
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
    <div class="bg-white rounded min-h-50 p-5 flex flex-col justify-between">
      <span class="text-xl font-bold">
        {{ message }}
      </span>
      <div class="flex justify-end mt-4">
        <button
          class="mx-2 px-4 py-2 shadow-sm/20 rounded transition-all"
          :class="{
            'bg-red-500 text-white  hover:bg-red-600 active:bg-red-800': type === 'warn',
            'bg-blue-500 text-white  hover:bg-blue-600 active:bg-blue-800': type === 'normal',
          }"
          @click="handleClick"
          type="button"
        >
          <b>{{ t('common.confirm') }}</b>
        </button>
        <button
          class="mx-2 px-4 py-2 shadow-sm/20 rounded hover:bg-gray-200 transition-all active:bg-gray-300"
          @click="closePopupPanel"
          type="button"
        >
          <b>{{ t('common.cancel') }}</b>
        </button>
      </div>
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
