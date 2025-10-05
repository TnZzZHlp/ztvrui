<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import NetworksComponent from '@/components/networks/NetworkList.vue'
import AddNetworkDialog from '@/components/networks/AddNetworkDialog.vue'
import ModifyInfoDialog from '@/components/networks/ModifyInfoDialog.vue'
import { Button } from '@/components/ui/button'

const { t } = useI18n()
const showAddDialog = ref(false)
const showModifyDialog = ref(false)
</script>

<template>
  <div class="max-h-screen">
    <!-- Header -->
    <header class="shadow h-[60px] xl:px-[30%]">
      <div class="h-full flex items-center justify-between px-4">
        <span class="font-bold text-xl">{{ t('network.default') }}</span>
        <div>
          <Button variant="outline" class="mx-2" @click="showModifyDialog = true">
            {{ t('auth.modifyInfo') }}
          </Button>
          <Button class="mx-2" @click="showAddDialog = true">
            {{ t('network.add.default') }}
          </Button>
        </div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="xl:px-[30%] h-full flex items-center flex-col justify-start p-极4">
      <NetworksComponent />
    </main>

    <!-- Dialogs -->
    <AddNetworkDialog v-model:open="showAddDialog" />
    <ModifyInfoDialog v-model:open="showModifyDialog" />
  </div>
</template>
