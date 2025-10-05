<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import _ from 'lodash'
import { useNetworkDetailStore } from '@/stores/networkDetail'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import { createOrUpdateNetwork } from '@/api/zerotier/controller'
import compile from './RulesCompile'
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
} from '@/components/ui/dialog'
import { Button } from '@/components/ui/button'
import type { ControllerNetworkInfo } from '@/types/zerotier/controller'
import { Textarea } from '@/components/ui/textarea'

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
const originalRules = ref('')

watch(
    () => props.networkId,
    (newValue) => {
        if (newValue) {
            const networkData = networkDetailStore.networksData.find((data) => data.id === newValue)
            if (networkData) {
                networkDataClone.value = _.cloneDeep(networkData)
                // Load original rules from web storage
                originalRules.value = localStorage.getItem('networkRules_' + newValue) || ''
            }
        }
    },
    { immediate: true }
)

const handleSubmit = () => {
    if (!networkDataClone.value) return

    // Compile rules
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
    createOrUpdateNetwork(networkDataClone.value.id as string, {
        ...networkDataClone.value,
        rules: rules,
        tags: tagsArray,
        capabilities: capsArray,
    })
        .then(() => {
            // Update the network data in the storage
            const event = new CustomEvent('controllerNetworkInfoUpdate', {
                detail: networkDataClone.value?.id,
            })
            eventBus.dispatchEvent(event)
            // Update the network data in the local storage
            localStorage.setItem('networkRules_' + networkDataClone.value?.id, originalRules.value)

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
</script>

<template>
    <Dialog :open="open" @update:open="handleOpenChange">
        <DialogContent class="sm:max-w-[800px] max-h-[80vh]">
            <DialogHeader>
                <DialogTitle>{{ t('network.rules.default') }}</DialogTitle>
                <DialogDescription class="text-orange-600 font-semibold">
                    {{ t('network.rules.warn') }}
                </DialogDescription>
            </DialogHeader>
            <div class="p-4 flex-1" v-if="networkDataClone">
                <Textarea v-model="originalRules" :placeholder="t('network.rules.default')"
                    class="max-h-[600px] h-[400px]"></Textarea>
            </div>
            <DialogFooter class="gap-2">
                <Button type="button" variant="outline" @click="emit('update:open', false)">
                    {{ t('common.cancel') }}
                </Button>
                <Button type="button" @click="handleSubmit">
                    {{ t('common.confirm') }}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
