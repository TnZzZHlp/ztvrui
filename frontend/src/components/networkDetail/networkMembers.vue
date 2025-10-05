<script setup lang="ts">
import {
    createOrUpdateNetworkMember,
    getNetworkMemberById,
    listNetworkMemberIds,
    deleteNetworkMember,
} from '@/api/zerotier/controller'
import type { ControllerNetworkMemberInfo } from '@/types/zerotier/controller'
import { showSnackBar } from '@/utils/showSnackBar'
import { computed, onBeforeMount, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { copyToClipboard } from '@/utils/copyToClipboard'
import yesIcon from '@/assets/icons/yes.svg'
import noIcon from '@/assets/icons/no.svg'
import settingIcon from '@/assets/icons/setting.svg'
import deleteIcon from '@/assets/icons/delete.svg'
import ModifyMemberIPDialog from './member/ModifyMemberIPDialog.vue'
import { eventBus } from '@/utils/eventBus'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import { Button } from '@/components/ui/button'

const route = useRoute()
const { t } = useI18n()

const members = ref<ControllerNetworkMemberInfo[]>([])

const searchKeyword = ref('')
const shouldShowMember = computed(() => {
    return (member: ControllerNetworkMemberInfo) => {
        if (!searchKeyword.value) return true
        const keyword = searchKeyword.value.toLowerCase()
        return (
            member.name.toLowerCase().includes(keyword) ||
            member.id.toLowerCase().includes(keyword) ||
            member.ipAssignments.some((ip) => ip.toLowerCase().includes(keyword))
        )
    }
})

// Confirm dialog state
const showConfirmDialog = ref(false)
const confirmMessage = ref('')
const confirmAction = ref<() => void>(() => { })

const openConfirmDialog = (message: string, action: () => void) => {
    confirmMessage.value = message
    confirmAction.value = action
    showConfirmDialog.value = true
}

// Modify member IP dialog state
const showModifyIPDialog = ref(false)
const selectedMember = ref<ControllerNetworkMemberInfo | null>(null)

const openModifyIPDialog = (member: ControllerNetworkMemberInfo) => {
    selectedMember.value = member
    showModifyIPDialog.value = true
}

const changeMemberSettings = (member: ControllerNetworkMemberInfo) => {
    // Function to change member settings
    const networkId = route.params.networkId as string

    createOrUpdateNetworkMember(networkId, member.id, member)
        .then(() => {
            showSnackBar(t('common.updateSuccess'), 'success')
        })
        .catch((err) => {
            showSnackBar(t('common.updateFailed') + err, 'error')
        })
}

const fetchMembers = () => {
    const networkId = route.params.networkId as string
    listNetworkMemberIds(networkId)
        .then((memberIds) => {
            Object.keys(memberIds).forEach((memberId) => {
                getNetworkMemberById(networkId, memberId).then((member) => {
                    if (member) {
                        if (!member.name) {
                            member.name = t('network.member.unnamed') // Default name if not set
                        }

                        const index = members.value.findIndex((m) => m.id === member.id)
                        if (index !== -1) {
                            // Update existing member
                            members.value[index] = member
                        } else {
                            // Add new member
                            members.value.push(member)
                        }
                    } else {
                        throw new Error(`Failed to get member with ID ${memberId}`)
                    }
                })
            })
        })
        .catch((error) => {
            showSnackBar(t('network.member.error') + error, 'error')
        })
}

onBeforeMount(() => {
    // Get network member ids
    fetchMembers()
    eventBus.addEventListener('networkMemberChanged', fetchMembers)
})
</script>

<template>
    <div class="flex flex-col gap-4 p-4">
        <div class="bg-gray-200 flex items-center justify-between p-2 rounded-lg">
            <input class="w-full h-full focus:outline-none" type="text" v-model="searchKeyword" />
            <img src="@/assets/icons/search.svg" alt="search" class="w-5 h-5" />
        </div>

        <!-- Member List -->
        <ol v-if="members.length !== 0" class="text-center">
            <li class="rounded-lg shadow min-h-30 mb-2" v-for="(member, index) in members" :key="member.id"
                v-show="shouldShowMember(member)">
                <!-- Member Info -->
                <div class="flex justify-between bg-gray-200 p-2 rounded-t-lg">
                    <input class="font-bold text-xl focus:outline-none border-b-1 border-gray-300" :width="200"
                        type="text" placeholder="Member Name" v-model="members[index].name" autocomplete="off"
                        @change="changeMemberSettings(members[index])" />
                    <div class="flex items-center gap-3">
                        <!-- Edit Button -->
                        <Button variant="secondary" size="icon" @click="openModifyIPDialog(member)">
                            <img :src="settingIcon" alt="editMember" class="w-5 h-5" />
                        </Button>

                        <!-- Delete Button -->
                        <Button variant="destructive" size="icon" @click="
                            openConfirmDialog(
                                t('network.member.deleteConfirm'),
                                () => {
                                    deleteNetworkMember(member.nwid, member.id)
                                        .then(() => {
                                            showSnackBar(t('network.member.deleteSuccess'), 'success')
                                            members.splice(index, 1) // Remove member from list
                                        })
                                        .catch((err) => {
                                            showSnackBar(t('network.member.deleteFailed') + err, 'error')
                                        })
                                },
                            )
                            ">
                            <img :src="deleteIcon" alt="editMember" class="filter brightness-0 invert w-3 h-3" />
                        </Button>

                        <!-- Authorization Button -->
                        <Button :variant="member.authorized ? 'default' : 'destructive'" size="icon" @click="
                            () => {
                                members[index].authorized = !members[index].authorized
                                changeMemberSettings(members[index])
                            }
                        ">
                            <img :src="member.authorized ? yesIcon : noIcon" alt="changeAuthorization"
                                class="filter brightness-0 invert w-3 h-3" />
                        </Button>
                    </div>
                </div>

                <!-- Device ID -->
                <div class="border-b-1 border-gray-300 flex items-center justify-between p-2">
                    <span class="font-bold">{{ t('network.member.id') }}</span>
                    <Button variant="outline" size="sm" @click="copyToClipboard(member.id)">
                        {{ member.id }}
                    </Button>
                </div>

                <!-- Device IP -->
                <div class="border-b-1 border-gray-300 flex items-center justify-between p-2">
                    <span class="font-bold">{{ t('network.ip') }}</span>
                    <div>
                        <Button variant="outline" size="sm" class="mr-1" v-for="ip in member.ipAssignments" :key="ip"
                            @click="copyToClipboard(ip)">
                            {{ ip }}
                        </Button>
                    </div>
                </div>
                <div class="flex items-center justify-evenly h-10">
                    <Button :variant="!member.noAutoAssignIps ? 'default' : 'destructive'" size="sm" @click="
                        () => {
                            member.noAutoAssignIps = !member.noAutoAssignIps
                            changeMemberSettings(member)
                        }
                    ">
                        {{ t('network.member.autoAssignIps') }}
                        <img :src="!member.noAutoAssignIps ? yesIcon : noIcon"
                            class="ml-1 w-4 h-4 inline-block filter brightness-0 invert" />
                    </Button>
                    <Button :variant="member.activeBridge ? 'default' : 'destructive'" size="sm" @click="
                        () => {
                            member.activeBridge = !member.activeBridge
                            changeMemberSettings(member)
                        }
                    ">
                        {{ t('network.member.activeBridge') }}
                        <img :src="member.activeBridge ? yesIcon : noIcon"
                            class="ml-1 w-4 h-4 inline-block filter brightness-0 invert" />
                    </Button>
                </div>
            </li>
        </ol>

        <!-- Dialogs -->
        <ConfirmDialog :open="showConfirmDialog" :message="confirmMessage" :onConfirm="confirmAction" type="warn"
            @update:open="showConfirmDialog = $event" />
        <ModifyMemberIPDialog :open="showModifyIPDialog" :member-info="selectedMember"
            @update:open="showModifyIPDialog = $event" />
    </div>
</template>
