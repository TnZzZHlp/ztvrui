<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import NetworksComponent from '@/components/networks/networkList.vue'
import { showAddNetworkPanel } from '@/components/networks/showAddNetworkPopupPanel'
import { showModifyInfoPanel } from '@/components/networks/showModifyInfoPopupPanel'
const { t } = useI18n()
</script>

<template>
  <div class="max-h-screen">
    <!-- Header -->
    <header class="shadow h-[60px] xl:px-[30%]">
      <div class="h-full flex items-center justify-between px-4">
        <span class="font-bold text-xl">{{ t('network.default') }}</span>
        <div>
          <button
            class="mx-2 px-4 py-2 hover:bg-gray-200 active:bg-gray-300 shadow-sm/20 rounded"
            @click="showModifyInfoPanel"
          >
            {{ t('auth.modifyInfo') }}
          </button>
          <button
            class="mx-2 px-4 py-2 hover:bg-gray-200 active:bg-gray-300 shadow-sm/20 rounded"
            @click="showAddNetworkPanel"
          >
            {{ t('network.add.default') }}
          </button>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="xl:px-[30%] h-full flex items-center flex-col justify-start p-4">
      <NetworksComponent />
    </main>
  </div>
</template>
