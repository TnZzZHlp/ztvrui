<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'

const { t } = useI18n()
const { networkData } = useCurrentNetwork()
</script>

<template>
  <div v-if="networkData" class="rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <p class="text-sm font-medium text-muted-foreground">{{ t('network.creationTime') }}</p>
    <p class="mt-2 text-2xl font-bold tracking-tight">
      {{ networkData.creationTime ? new Date(networkData.creationTime).toLocaleString() : t('common.noData') }}
    </p>
  </div>
</template>
