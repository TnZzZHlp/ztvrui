<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Settings } from 'lucide-vue-next'
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { Button } from '@/components/ui/button'
import ModifyRulesDialog from './ModifyRulesDialog.vue'

const { t } = useI18n()
const { networkData } = useCurrentNetwork()

const showDialog = ref(false)
</script>

<template>
  <div v-if="networkData" class="rounded-xl border bg-card p-5 text-card-foreground shadow-xs">
    <div class="mb-3 flex items-center justify-between">
      <h2 class="text-base font-semibold">{{ t('network.rules.default') }}</h2>
      <Button
        type="button"
        variant="ghost"
        size="icon"
        :aria-label="t('network.rules.default')"
        @click="showDialog = true"
      >
        <Settings class="h-4 w-4" />
      </Button>
    </div>

    <p class="text-sm text-muted-foreground">
      {{ t('network.rules.activeCount', { count: networkData.rules?.length ?? 0 }) }}
    </p>

    <ModifyRulesDialog
      :open="showDialog"
      :network-id="networkData.id ?? null"
      @update:open="showDialog = $event"
    />
  </div>
</template>
