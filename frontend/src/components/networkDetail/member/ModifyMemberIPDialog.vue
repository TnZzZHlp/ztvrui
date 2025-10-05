<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import _ from 'lodash'
import type { ControllerNetworkMemberInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { eventBus } from '@/utils/eventBus'
import { createOrUpdateNetworkMember } from '@/api/zerotier/controller'
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

const { t } = useI18n()

interface Props {
    open: boolean
    memberInfo: ControllerNetworkMemberInfo | null
}

const props = defineProps<Props>()
const emit = defineEmits<{
    'update:open': [value: boolean]
}>()

const memberInfoClone = ref<ControllerNetworkMemberInfo | null>(null)

watch(() => props.memberInfo, (newValue) => {
    if (newValue) {
        memberInfoClone.value = _.cloneDeep(newValue)
    }
}, { immediate: true })

const handleSubmit = () => {
    if (!memberInfoClone.value) return

    const ipv4Regex =
        /^(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$/

    const ipv6Regex =
        /^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$|^([0-9a-fA-F]{1,4}:){1,7}:$|^([0-9a-fA-F]{1,4}:){1,6}:[0-9a-fA-F]{1,4}$|^([0-9a-fA-F]{1,4}:){1,5}(:[0-9a-fA-F]{1,4}){1,2}$|^([0-9a-fA-F]{1,4}:){1,4}(:[0-9a-fA-F]{1,4}){1,3}$|^([0-9a-fA-F]{1,4}:){1,3}(:[0-9a-fA-F]{1,4}){1,4}$|^([0-9a-fA-F]{1,4}:){1,2}(:[0-9a-fA-F]{1,4}){1,5}$|^[0-9a-fA-F]{1,4}:((:[0-9a-fA-F]{1,4}){6}|:)$|^:((:[0-9a-fA-F]{1,4}){7}|:)$|^:((:[0-9a-fA-F]{1,4}){6}|:)$/

    const ipAssignments = memberInfoClone.value.ipAssignments

    // Validate IP addresses
    for (const ip of ipAssignments) {
        if (ip && !ipv4Regex.test(ip) && !ipv6Regex.test(ip)) {
            showSnackBar(t('network.ipAssignmentPools.invalidedIP'), 'error')
            return
        }
    }

    const controllerNetworkMemberSettings = {
        ...memberInfoClone.value,
        ipAssignments: ipAssignments,
    }

    // Update member info
    createOrUpdateNetworkMember(
        memberInfoClone.value.nwid,
        memberInfoClone.value.id,
        controllerNetworkMemberSettings,
    )
        .then(() => {
            eventBus.dispatchEvent(new Event('networkMemberChanged'))
            showSnackBar(t('common.updateSuccess'), 'success')
            emit('update:open', false)
        })
        .catch((error) => {
            showSnackBar(error.message, 'error')
        })
}

const handleOpenChange = (newOpen: boolean) => {
    emit('update:open', newOpen)
}

const addIP = () => {
    if (memberInfoClone.value) {
        memberInfoClone.value.ipAssignments.push('')
    }
}

const removeIP = (index: number) => {
    if (memberInfoClone.value) {
        memberInfoClone.value.ipAssignments.splice(index, 1)
    }
}
</script>

<template>
    <Dialog :open="open" @update:open="handleOpenChange">
        <DialogContent class="sm:max-w-[500px]">
            <DialogHeader>
                <DialogTitle>{{ t('network.member.modifyIP') }}</DialogTitle>
                <DialogDescription>
                    {{ t('network.ip') }}
                </DialogDescription>
            </DialogHeader>
            <div class="py-4" v-if="memberInfoClone">
                <h4 class="font-semibold mb-3">{{ t('network.ip') }}</h4>
                <ul class="space-y-2">
                    <li v-for="(_, index) in memberInfoClone.ipAssignments" :key="index"
                        class="flex items-center gap-2">
                        <input type="text" v-model="memberInfoClone.ipAssignments[index]"
                            class="flex-1 border p-2 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                            :placeholder="t('network.ip')" />
                        <Button type="button" variant="destructive" size="icon" @click="removeIP(index)">
                            <img :src="deleteIcon" alt="delete" class="w-4 h-4 filter brightness-0 invert" />
                        </Button>
                    </li>
                </ul>
            </div>
            <DialogFooter class="gap-2">
                <Button type="button" variant="outline" @click="emit('update:open', false)">
                    {{ t('common.cancel') }}
                </Button>
                <Button type="button" variant="secondary" @click="addIP">
                    {{ t('common.add') }}
                </Button>
                <Button type="button" @click="handleSubmit">
                    {{ t('common.confirm') }}
                </Button>
            </DialogFooter>
        </DialogContent>
    </Dialog>
</template>
