<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'

const networkDetailStore = useNetworkDetailStore()
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'

const route = useRoute()
const { t } = useI18n()

const networkData = computed(() => {
    return networkDetailStore.networksData.find((data) => data.id === (route.params.networkId as string))
})
</script>

<template>
    <!-- Network Creation Time -->
    <div v-if="networkData" class="p-4 shadow bg-white rounded-2lg">
        <p class="text-gray-500">{{ t('network.creationTime') }}</p>
        <h1 class="text-3xl font-bold">
            {{ new Date(networkData?.creationTime as number).toLocaleString() }}
        </h1>
    </div>
</template>
