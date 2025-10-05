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
import settingIcon from '@/assets/icons/setting.svg'
import deleteIcon from '@/assets/icons/delete.svg'
import ModifyMemberIPDialog from './member/ModifyMemberIPDialog.vue'
import { eventBus } from '@/utils/eventBus'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Switch } from '@/components/ui/switch'

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
    <!-- Search Bar -->
    <div class="flex items-center gap-2">
      <Input v-model="searchKeyword" :placeholder="t('common.search')" class="flex-1" />
    </div>

    <!-- Member Table -->
    <div class="rounded-md border">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead class="w-[200px]">{{ t('network.member.name') }}</TableHead>
            <TableHead>{{ t('network.member.id') }}</TableHead>
            <TableHead>{{ t('network.ip') }}</TableHead>
            <TableHead class="text-center">{{ t('network.member.authorized') }}</TableHead>
            <TableHead class="text-center">{{ t('network.member.autoAssignIps') }}</TableHead>
            <TableHead class="text-center">{{ t('network.member.activeBridge') }}</TableHead>
            <TableHead class="text-center w-[120px]">{{ t('common.actions') }}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-if="members.length === 0">
            <TableCell :colspan="7" class="h-24 text-center">
              {{ t('network.member.noMembers') }}
            </TableCell>
          </TableRow>
          <TableRow v-for="(member, index) in members" :key="member.id" v-show="shouldShowMember(member)">
            <!-- Name -->
            <TableCell>
              <Input v-model="members[index].name" :placeholder="t('network.member.unnamed')" class="h-8 min-w-[120px]"
                @change="changeMemberSettings(members[index])" />
            </TableCell>

            <!-- Member ID -->
            <TableCell>
              <Button variant="ghost" size="sm" @click="copyToClipboard(member.id)">
                {{ member.id }}
              </Button>
            </TableCell>

            <!-- IP Addresses -->
            <TableCell>
              <div class="flex flex-wrap gap-1">
                <Button v-for="ip in member.ipAssignments" :key="ip" variant="outline" size="sm"
                  @click="copyToClipboard(ip)">
                  {{ ip }}
                </Button>
                <span v-if="member.ipAssignments.length === 0" class="text-sm text-muted-foreground">-</span>
              </div>
            </TableCell>

            <!-- Authorized -->
            <TableCell class="text-center">
              <Switch :model-value="member.authorized" @update:model-value="
                (checked: boolean) => {
                  members[index].authorized = checked
                  changeMemberSettings(members[index])
                }
              " />
            </TableCell>

            <!-- Auto Assign IPs -->
            <TableCell class="text-center">
              <Switch :model-value="!member.noAutoAssignIps" @update:model-value="
                (checked: boolean) => {
                  members[index].noAutoAssignIps = !checked
                  changeMemberSettings(members[index])
                }
              " />
            </TableCell>

            <!-- Active Bridge -->
            <TableCell class="text-center">
              <Switch :model-value="member.activeBridge" @update:model-value="
                (checked: boolean) => {
                  members[index].activeBridge = checked
                  changeMemberSettings(members[index])
                }
              " />
            </TableCell>

            <!-- Actions -->
            <TableCell class="text-center">
              <div class="flex items-center justify-center gap-1">
                <!-- Edit Button -->
                <Button variant="ghost" size="icon" @click="openModifyIPDialog(member)">
                  <img :src="settingIcon" alt="editMember" class="w-4 h-4" />
                </Button>

                <!-- Delete Button -->
                <Button variant="ghost" size="icon" @click="
                  openConfirmDialog(
                    t('network.member.deleteConfirm'),
                    () => {
                      deleteNetworkMember(member.nwid, member.id)
                        .then(() => {
                          showSnackBar(t('network.member.deleteSuccess'), 'success')
                          members.splice(index, 1)
                        })
                        .catch((err) => {
                          showSnackBar(t('network.member.deleteFailed') + err, 'error')
                        })
                    },
                  )
                  ">
                  <img :src="deleteIcon" alt="deleteMember" class="w-4 h-4 text-destructive" />
                </Button>
              </div>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <!-- Dialogs -->
    <ConfirmDialog :open="showConfirmDialog" :message="confirmMessage" :onConfirm="confirmAction" type="warn"
      @update:open="showConfirmDialog = $event" />
    <ModifyMemberIPDialog :open="showModifyIPDialog" :member-info="selectedMember"
      @update:open="showModifyIPDialog = $event" />
  </div>
</template>
