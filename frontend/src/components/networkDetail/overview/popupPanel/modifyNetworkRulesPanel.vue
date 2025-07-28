<script setup lang="ts">
import { onBeforeMount, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { networksData } from '@/components/networkDetail/networkDetailStorage'
import compile from './rulesCompile'
import _ from 'lodash'
import { showSnackBar } from '@/utils/showSnackBar'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import { eventBus } from '@/utils/eventBus'

const { container, networkId } = defineProps({
  container: HTMLDivElement,
  networkId: String,
})
const networkData = ref(
  _.cloneDeep(networksData.value.find((data) => data.id === (networkId as string))),
)

const close = ref(false)
const { t } = useI18n()
const originalRules = ref('')

const handleClick = () => {
  // Save original rules to web storage

  const rules: never[] = []
  const tags = {}
  const caps = {}

  const result = compile(originalRules.value, rules, caps, tags)

  if (result) {
    const [first, second, error] = result
    showSnackBar('row:' + first + ' ' + 'col:' + second + ' ' + error, 'error')
    return
  }

  const capsArray = []
  const capabilitiesByName = {}
  for (const n in caps) {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    capsArray.push(caps[n])

    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    capabilitiesByName[n] = caps[n].id
  }
  const tagsArray = []
  for (const n in tags) {
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore
    const t = tags[n]
    const dfl = t['default']
    tagsArray.push({
      id: t.id,
      default: dfl || dfl === 0 ? dfl : null,
    })
  }

  // Update network data
  createOrUpdateNetwork(networkData.value?.id as string, {
    ...networkData.value,
    name: networkData.value?.name as string,
    rules: rules,
    tags: tagsArray,
    capabilities: capsArray,
  })
    .then(() => {
      // Update the network data in the storage
      const event = new CustomEvent('controllerNetworkInfoUpdate', {
        detail: networkData.value?.id,
      })
      eventBus.dispatchEvent(event)
      // Update the network data in the local storage
      localStorage.setItem('networkRules_' + networkData.value?.id, originalRules.value)

      showSnackBar(t('common.updateSuccess'), 'success')
    })
    .catch((err) => {
      showSnackBar(t('common.updateFailed') + err, 'error')
    })
  closePopupPanel()
}

const closePopupPanel = () => {
  close.value = true
  setTimeout(() => {
    container?.remove()
  }, 300)
}

onBeforeMount(() => {
  // Load original rules from web storage
  originalRules.value = localStorage.getItem('networkRules_' + networkData.value?.id) || ''
})
</script>

<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 popup-panel"
    :class="{ 'popup-panel-leave-active': close }"
  >
    <div
      class="bg-white rounded p-5 flex flex-col justify-between w-9/10 h-8/10 lg:w-2/3 shadow-lg"
    >
      <!-- Rules -->
      <div class="flex flex-col h-full">
        <div class="flex items-center justify-between">
          <span class="text-xl text-nowrap">{{ t('network.rules.default') }}</span>
          <span class="w-full font-bold text-end">{{ t('network.rules.warn') }}</span>
        </div>
        <textarea
          name="rules"
          id="rules"
          class="border rounded p-2 h-full resize-none"
          v-model="originalRules"
        ></textarea>
      </div>

      <!-- Actions -->
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
