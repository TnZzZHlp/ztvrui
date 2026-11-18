<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Copy, RefreshCw, Search, Settings2, Trash2, UserPlus } from 'lucide-vue-next'
import {
  createOrUpdateNetworkMember,
  deleteNetworkMember,
  getNetworkMemberById,
  listNetworkMemberIds,
} from '@/api/zerotier/controller'
import type {
  ControllerNetworkMemberInfo,
  ControllerNetworkMemberSettings,
} from '@/types/zerotier/controller'
import { getErrorMessage, showSnackBar } from '@/utils/showSnackBar'
import { copyToClipboard } from '@/utils/copyToClipboard'
import { eventBus } from '@/utils/eventBus'
import ConfirmDialog from '@/components/ConfirmDialog.vue'
import AddMemberDialog from './member/AddMemberDialog.vue'
import ModifyMemberIPDialog from './member/ModifyMemberIPDialog.vue'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Switch } from '@/components/ui/switch'

type StatusFilter = 'all' | 'authorized' | 'unauthorized'

const route = useRoute()
const { t } = useI18n()

const members = ref<ControllerNetworkMemberInfo[]>([])
const savedNames = new Map<string, string>()
const editingMemberId = ref<string | null>(null)
const loading = ref(false)
const isFetching = ref(false)
let pollTimer: number | undefined

const searchKeyword = ref('')
const statusFilter = ref<StatusFilter>('all')

const totalCount = computed(() => members.value.length)
const authorizedCount = computed(() => members.value.filter((m) => m.authorized).length)
const unauthorizedCount = computed(() => members.value.filter((m) => !m.authorized).length)

const filteredMembers = computed(() => {
  const keyword = searchKeyword.value.trim().toLowerCase()
  return members.value.filter((member) => {
    if (statusFilter.value === 'authorized' && !member.authorized) return false
    if (statusFilter.value === 'unauthorized' && member.authorized) return false
    if (!keyword) return true

    return (
      (member.name || '').toLowerCase().includes(keyword) ||
      member.id.toLowerCase().includes(keyword) ||
      (member.ipAssignments ?? []).some((ip) => ip.toLowerCase().includes(keyword))
    )
  })
})

const showConfirmDialog = ref(false)
const confirmMessage = ref('')
const confirmAction = ref<() => void>(() => {})

const openConfirmDialog = (message: string, action: () => void) => {
  confirmMessage.value = message
  confirmAction.value = action
  showConfirmDialog.value = true
}

const showAddMemberDialog = ref(false)
const showModifyIPDialog = ref(false)
const selectedMember = ref<ControllerNetworkMemberInfo | null>(null)

const openModifyIPDialog = (member: ControllerNetworkMemberInfo) => {
  selectedMember.value = member
  showModifyIPDialog.value = true
}

const replaceMember = (updated: ControllerNetworkMemberInfo) => {
  const normalized: ControllerNetworkMemberInfo = {
    ...updated,
    name: updated.name ?? '',
  }
  savedNames.set(normalized.id, normalized.name)
  const index = members.value.findIndex((m) => m.id === normalized.id)
  if (index !== -1) {
    members.value[index] = normalized
  } else {
    members.value.push(normalized)
  }
}

const updateMemberSetting = async (
  member: ControllerNetworkMemberInfo,
  patch: Partial<ControllerNetworkMemberSettings>,
) => {
  const networkId = route.params.networkId as string
  try {
    const updated = await createOrUpdateNetworkMember(networkId, member.id, patch)
    replaceMember(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}

const handleNameFocus = (memberId: string) => {
  editingMemberId.value = memberId
}

const handleNameBlur = async (member: ControllerNetworkMemberInfo) => {
  if (editingMemberId.value === member.id) {
    editingMemberId.value = null
  }
  const trimmed = (member.name ?? '').trim()
  const previous = savedNames.get(member.id) ?? ''
  if (trimmed === previous) {
    member.name = trimmed
    return
  }

  const networkId = route.params.networkId as string
  try {
    const updated = await createOrUpdateNetworkMember(networkId, member.id, { name: trimmed })
    replaceMember(updated)
    showSnackBar(t('common.updateSuccess'), 'success')
  } catch (err) {
    member.name = previous
    showSnackBar(`${t('common.updateFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}

const handleDeleteMember = async (member: ControllerNetworkMemberInfo) => {
  try {
    await deleteNetworkMember(member.nwid, member.id)
    members.value = members.value.filter((item) => item.id !== member.id)
    savedNames.delete(member.id)
    showSnackBar(t('network.member.deleteSuccess'), 'success')
  } catch (err) {
    showSnackBar(`${t('network.member.deleteFailed')}: ${getErrorMessage(err)}`, 'error')
  }
}

const fetchMembers = async (showErrors = false) => {
  const networkId = route.params.networkId as string
  if (!networkId || isFetching.value) return

  isFetching.value = true
  if (members.value.length === 0) {
    loading.value = true
  }

  try {
    const memberIdsObj = await listNetworkMemberIds(networkId)
    const ids = Object.keys(memberIdsObj)
    const results = await Promise.allSettled(ids.map((id) => getNetworkMemberById(networkId, id)))

    const draftById = new Map<string, string>()
    if (editingMemberId.value) {
      const active = members.value.find((m) => m.id === editingMemberId.value)
      if (active) {
        draftById.set(active.id, active.name ?? '')
      }
    }

    const nextMembers: ControllerNetworkMemberInfo[] = []
    for (const res of results) {
      if (res.status === 'fulfilled' && res.value) {
        const normalized: ControllerNetworkMemberInfo = {
          ...res.value,
          name: res.value.name ?? '',
        }
        savedNames.set(normalized.id, normalized.name)
        const activeDraft = draftById.get(normalized.id)
        if (activeDraft !== undefined) {
          normalized.name = activeDraft
        }
        nextMembers.push(normalized)
      }
    }

    nextMembers.sort(
      (a, b) => (a.creationTime ?? 0) - (b.creationTime ?? 0) || a.id.localeCompare(b.id),
    )
    members.value = nextMembers
  } catch (error) {
    if (showErrors) {
      showSnackBar(`${t('network.member.error')}: ${getErrorMessage(error)}`, 'error')
    }
  } finally {
    loading.value = false
    isFetching.value = false
  }
}

const handleMemberChanged = () => {
  void fetchMembers(true)
}

watch(
  () => route.params.networkId,
  (newId, oldId) => {
    if (newId && newId !== oldId) {
      members.value = []
      savedNames.clear()
      void fetchMembers(true)
    }
  },
)

onMounted(() => {
  void fetchMembers(true)
  eventBus.addEventListener('networkMemberChanged', handleMemberChanged)
  pollTimer = window.setInterval(() => {
    if (document.visibilityState === 'visible') {
      void fetchMembers(false)
    }
  }, 5000)
})

onBeforeUnmount(() => {
  eventBus.removeEventListener('networkMemberChanged', handleMemberChanged)
  if (pollTimer !== undefined) {
    clearInterval(pollTimer)
  }
})
</script>

<template>
  <div class="flex flex-col gap-4 p-2 sm:p-4">
    <div class="flex flex-wrap items-center justify-between gap-2">
      <div class="relative min-w-[220px] flex-1">
        <Search class="pointer-events-none absolute top-1/2 left-3 h-4 w-4 -translate-y-1/2 text-muted-foreground" />
        <Input v-model="searchKeyword" :placeholder="t('common.search')" class="pl-9" />
      </div>

      <div class="flex flex-wrap items-center gap-1.5">
        <Button
          type="button"
          size="sm"
          :variant="statusFilter === 'all' ? 'default' : 'outline'"
          @click="statusFilter = 'all'"
        >
          {{ t('common.all') }} ({{ totalCount }})
        </Button>
        <Button
          type="button"
          size="sm"
          :variant="statusFilter === 'authorized' ? 'default' : 'outline'"
          @click="statusFilter = 'authorized'"
        >
          {{ t('network.member.authorized') }} ({{ authorizedCount }})
        </Button>
        <Button
          type="button"
          size="sm"
          :variant="statusFilter === 'unauthorized' ? 'default' : 'outline'"
          @click="statusFilter = 'unauthorized'"
        >
          {{ t('network.member.unauthorized') }} ({{ unauthorizedCount }})
        </Button>
      </div>

      <div class="flex items-center gap-2">
        <Button
          type="button"
          variant="outline"
          size="sm"
          :disabled="isFetching"
          @click="fetchMembers(true)"
        >
          <RefreshCw class="h-4 w-4" :class="{ 'animate-spin': isFetching }" />
          <span class="hidden sm:inline">{{ t('common.refresh') }}</span>
        </Button>
        <Button type="button" size="sm" @click="showAddMemberDialog = true">
          <UserPlus class="h-4 w-4" />
          <span>{{ t('network.member.add') }}</span>
        </Button>
      </div>
    </div>

    <div class="overflow-x-auto rounded-xl border bg-card shadow-xs">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead class="w-[200px]">{{ t('network.member.name') }}</TableHead>
            <TableHead>{{ t('network.member.id') }}</TableHead>
            <TableHead>{{ t('network.ip') }}</TableHead>
            <TableHead class="text-center">{{ t('network.member.authorized') }}</TableHead>
            <TableHead class="text-center">{{ t('network.member.autoAssignIps') }}</TableHead>
            <TableHead class="text-center">{{ t('network.member.activeBridge') }}</TableHead>
            <TableHead class="w-[120px] text-center">{{ t('common.actions') }}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow v-if="loading">
            <TableCell :colspan="7" class="h-24 text-center text-muted-foreground">
              {{ t('common.loading') }}
            </TableCell>
          </TableRow>
          <TableRow v-else-if="members.length === 0">
            <TableCell :colspan="7" class="h-24 text-center text-muted-foreground">
              {{ t('network.member.noMembers') }}
            </TableCell>
          </TableRow>
          <TableRow v-else-if="filteredMembers.length === 0">
            <TableCell :colspan="7" class="h-24 text-center text-muted-foreground">
              {{ t('common.noData') }}
            </TableCell>
          </TableRow>
          <TableRow v-for="member in filteredMembers" :key="member.id">
            <TableCell>
              <Input
                v-model="member.name"
                :placeholder="t('network.member.unnamed')"
                class="h-8 min-w-[140px]"
                @focus="handleNameFocus(member.id)"
                @blur="handleNameBlur(member)"
                @keydown.enter="($event.target as HTMLInputElement).blur()"
              />
            </TableCell>

            <TableCell>
              <Button
                type="button"
                variant="ghost"
                size="sm"
                class="font-mono text-xs"
                @click="copyToClipboard(member.id)"
              >
                <span>{{ member.id }}</span>
                <Copy class="h-3 w-3" />
              </Button>
            </TableCell>

            <TableCell>
              <div class="flex flex-wrap gap-1">
                <Button
                  v-for="ip in member.ipAssignments"
                  :key="ip"
                  type="button"
                  variant="outline"
                  size="sm"
                  class="font-mono text-xs"
                  @click="copyToClipboard(ip)"
                >
                  {{ ip }}
                </Button>
                <span
                  v-if="!member.ipAssignments || member.ipAssignments.length === 0"
                  class="text-sm text-muted-foreground"
                >
                  -
                </span>
              </div>
            </TableCell>

            <TableCell class="text-center">
              <Switch
                :model-value="Boolean(member.authorized)"
                @update:model-value="(checked: boolean) => updateMemberSetting(member, { authorized: checked })"
              />
            </TableCell>

            <TableCell class="text-center">
              <Switch
                :model-value="!member.noAutoAssignIps"
                @update:model-value="
                  (checked: boolean) => updateMemberSetting(member, { noAutoAssignIps: !checked })
                "
              />
            </TableCell>

            <TableCell class="text-center">
              <Switch
                :model-value="Boolean(member.activeBridge)"
                @update:model-value="
                  (checked: boolean) => updateMemberSetting(member, { activeBridge: checked })
                "
              />
            </TableCell>

            <TableCell class="text-center">
              <div class="flex items-center justify-center gap-1">
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  :aria-label="t('network.member.modifyIP')"
                  @click="openModifyIPDialog(member)"
                >
                  <Settings2 class="h-4 w-4" />
                </Button>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon"
                  class="text-destructive hover:text-destructive"
                  :aria-label="t('common.delete')"
                  @click="
                    openConfirmDialog(t('network.member.deleteConfirm'), () => {
                      void handleDeleteMember(member)
                    })
                  "
                >
                  <Trash2 class="h-4 w-4" />
                </Button>
              </div>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>

    <ConfirmDialog
      :open="showConfirmDialog"
      :message="confirmMessage"
      :on-confirm="confirmAction"
      type="warn"
      @update:open="showConfirmDialog = $event"
    />
    <AddMemberDialog
      :open="showAddMemberDialog"
      :network-id="(route.params.networkId as string) || ''"
      @update:open="showAddMemberDialog = $event"
      @added="fetchMembers(true)"
    />
    <ModifyMemberIPDialog
      :open="showModifyIPDialog"
      :member-info="selectedMember"
      @update:open="showModifyIPDialog = $event"
    />
  </div>
</template>
