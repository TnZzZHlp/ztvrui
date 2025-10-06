<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import _ from 'lodash'
import { useNetworkDetailStore } from '@/stores/NetworkDetail'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import deleteIcon from '@/assets/icons/delete.svg'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { Input } from '@/components/ui/input'

const { t } = useI18n()
const networkDetailStore = useNetworkDetailStore()

interface Props {
    open: boolean
    networkId: string | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
    'update:open': [value: boolean]
}>()

const networkDataClone = ref<ControllerNetworkInfo | null>(null)

watch(
    () => props.networkId,
    (newValue) => {
        if (newValue) {
            const networkData = networkDetailStore.networksData.find((data) => data.id === newValue)
            if (networkData) {
                networkDataClone.value = _.cloneDeep(networkData)
            }
        }
    },
    { immediate: true }
)

const handleSubmit = () => {
    if (!networkDataClone.value) return

    // Validate IP range with regex
    const ipAssignmentPools = networkDataClone.value.ipAssignmentPools || []

    const ipRegex = /^(25[0-5]|2[0-4]\d|[01]?\d?\d)(\.(25[0-5]|2[0-4]\d|[01]?\d?\d)){3}$/

    const isValid = ipAssignmentPools.every((pool) => {
        return ipRegex.test(pool.ipRangeStart) && ipRegex.test(pool.ipRangeEnd)
    })

    if (!isValid) {
        showSnackBar(t('invalidedIP'), 'error')
        return
    }

    // Submit changes
    createOrUpdateNetwork(networkDataClone.value.id as string, {
        ...networkDataClone.value,
        ipAssignmentPools: networkDataClone.value.ipAssignmentPools,
    })
        .then(() => {
            // Update the network data in the storage
            const event = new CustomEvent('controllerNetworkInfoUpdate', {
                detail: networkDataClone.value?.id,
            })
            eventBus.dispatchEvent(event)
            showSnackBar(t('common.updateSuccess'), 'success')
            emit('update:open', false)
        })
        .catch((err) => {
            showSnackBar(t('common.updateFailed') + err, 'error')
        })
}

const handleOpenChange = (newOpen: boolean) => {
    emit('update:open', newOpen)
}

const addIPPool = () => {
    if (networkDataClone.value) {
        if (!networkDataClone.value.ipAssignmentPools) {
            networkDataClone.value.ipAssignmentPools = []
        }
        networkDataClone.value.ipAssignmentPools.push({
            ipRangeStart: '',
            ipRangeEnd: '',
        })
    }
}

const removeIPPool = (index: number) => {
    if (networkDataClone.value && networkDataClone.value.ipAssignmentPools) {
        networkDataClone.value.ipAssignmentPools.splice(index, 1)
    }
}
</script>

<template>
    <Dialog :open="open" @update:open="handleOpenChange">
        <DialogContent class="sm:max-w-[600px]">
            <DialogHeader>
                <DialogTitle>{{ t('network.ipAssignmentPools.default') }}</DialogTitle>
                <DialogDescription>
                    {{ t('network.ipAssignmentPools.ipRangeStart') }} / {{ t('network.ipAssignmentPools.ipRangeEnd') }}
                </DialogDescription>
            </DialogHeader>
            <div class="py-4" v-if="networkDataClone">
                <div class="space-y-3">
                    <div v-for="(pool, index) in networkDataClone.ipAssignmentPools" :key="index"
                        class="flex items-center gap-2">
                        <Input type="text" v-model="pool.ipRangeStart"
                            :placeholder="t('network.ipAssignmentPools.ipRangeStart')"
                            class="flex-1 border p-2 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                            autocomplete="off" />
                        <Input type="text" v-model="pool.ipRangeEnd"
                            :placeholder="t('network.ipAssignmentPools.ipRangeEnd')"
                            class="flex-1 border p-2 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                            autocomplete="off" />
                        <Button type="button" variant="destructive" size="icon" @click="removeIPPool(index)">
                            <img :src="deleteIcon" alt="delete" class="w-4 h-4 filter brightness-0 invert" />
                        </Button>
                    </div>
                    <div v-if="!networkDataClone.ipAssignmentPools || networkDataClone.ipAssignmentPools.length === 0"
                        class="text-center text-gray-500 py-4">
                        {{ t('common.noData') }}
                    </div>
                </div>
            </div>
            <DialogFooter class="gap-2">
                <Button type="button" variant="outline" @click="emit('update:open', false)">
                    {{ t('common.cancel') }}
                </Button>
                <Button type="button" variant="secondary" @click="addIPPool">
                    {{ t('common.add') }}
                </Button>
                <Button type="button" @click="handleSubmit">
                    {{ t('common.confirm') }}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
