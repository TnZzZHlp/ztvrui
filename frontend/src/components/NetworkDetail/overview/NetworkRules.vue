<script setup lang="ts">
import { useCurrentNetwork } from '@/composables/useCurrentNetwork'
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ModifyRulesDialog from './ModifyRulesDialog.vue'

const { t } = useI18n()
const { networkData } = useCurrentNetwork()

// Dialog state
const showDialog = ref(false)
const openDialog = () => {
    showDialog.value = true
}
</script>

<template>
    <!-- Network Rules -->
    <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
        <div class="flex justify-between mb-4">
            <span>{{ t('network.rules.default') }}</span>
            <button class="transition-all rounded hover:bg-gray-200 active:bg-gray-400" @click="openDialog">
                <img src="@/assets/icons/setting.svg" alt="Setting" class="h-6 object-contain" />
            </button>
        </div>

        <!-- Dialog -->
        <ModifyRulesDialog :open="showDialog" :network-id="networkData?.id ?? null" @update:open="showDialog = $event" />
    </div>
</template>
