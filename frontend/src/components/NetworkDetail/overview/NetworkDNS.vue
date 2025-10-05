<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/networkDetail'
import { Button } from '@/components/ui/button'
import { Separator } from '@/components/ui/separator'
import { Settings } from 'lucide-vue-next'
import { computed, ref } from 'vue'
import { useI18n } from 'vue-i18n'
import ModifyDNSDialog from './ModifyDNSDialog.vue'

const networkDetailStore = useNetworkDetailStore()
const { t } = useI18n()

const route = useRoute()
const networkData = computed(() => {
    return networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string))
})

// Dialog state
const showDialog = ref(false)
const selectedNetworkId = ref<string | null>(null)

const openDialog = () => {
    selectedNetworkId.value = networkData.value?.id || null
    showDialog.value = true
}
</script>

<template>
    <!-- Network DNS -->
    <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg space-y-4">
        <div class="flex items-center justify-between">
            <h2 class="text-lg font-semibold text-gray-900">DNS</h2>
            <Button variant="ghost" size="icon" @click="openDialog" aria-label="DNS Settings">
                <Settings class="h-4 w-4" />
            </Button>
        </div>

        <Separator />

        <div class="space-y-3">
            <div>
                <p class="text-sm font-medium text-muted-foreground mb-1">
                    {{ t('network.dns.domain') }}
                </p>
                <p class="text-base font-medium break-all">
                    {{ networkData?.dns?.domain || t('common.noData') }}
                </p>
            </div>

            <Separator />

            <div>
                <p class="text-sm font-medium text-muted-foreground mb-2">
                    {{ t('network.dns.servers') }}
                </p>
                <ol v-if="networkData?.dns?.servers && networkData.dns.servers.length > 0" class="space-y-1">
                    <li v-for="(server, index) in networkData.dns.servers" :key="index"
                        class="text-base font-mono text-gray-700 pl-4 relative before:content-['•'] before:absolute before:left-0">
                        {{ server }}
                    </li>
                </ol>
                <p v-else class="text-base ">
                    {{ t('common.noData') || '-' }}
                </p>
            </div>
        </div>

        <!-- Dialog -->
        <ModifyDNSDialog :open="showDialog" :network-id="selectedNetworkId" @update:open="showDialog = $event" />
    </div>
</template>
