<template>
  <AlertDialog :open="open" @update:open="updateOpen">
    <AlertDialogTrigger as-child>
      <Button variant="destructive" type="button" class="mx-2">
        {{ t('network.delete.default') }}
      </Button>
    </AlertDialogTrigger>
    <AlertDialogContent @escape-key-down="onEscape">
      <AlertDialogHeader>
        <AlertDialogTitle>{{ t('network.delete.default') }}</AlertDialogTitle>
        <AlertDialogDescription>{{ t('network.delete.notice') }}</AlertDialogDescription>
      </AlertDialogHeader>
      <form class="grid gap-4" @submit.prevent="confirmDelete">
        <Button
          variant="outline"
          type="button"
          class="justify-start font-mono"
          :aria-label="t('network.delete.copyId')"
          @click="copyToClipboard(networkId)"
        >
          {{ networkId }}
        </Button>
        <div class="grid gap-2">
          <label :for="idInputId">{{ t('network.delete.confirmId') }}</label>
          <Input
            :id="idInputId"
            v-model="confirmationId"
            autocomplete="off"
            spellcheck="false"
            :disabled="deleting"
            :aria-invalid="confirmationId.length > 0 && !idMatches"
            required
          />
        </div>
        <div class="grid gap-2">
          <label :for="passwordInputId">{{ t('auth.password') }}</label>
          <Input
            :id="passwordInputId"
            v-model="password"
            type="password"
            autocomplete="current-password"
            :disabled="deleting"
            required
          />
        </div>
        <p v-if="errorMessage" role="alert" class="text-sm text-destructive">{{ errorMessage }}</p>
        <AlertDialogFooter>
          <AlertDialogCancel :disabled="deleting">{{ t('common.cancel') }}</AlertDialogCancel>
          <Button variant="destructive" type="submit" :disabled="!canDelete">
            {{ t('common.confirm') }}
          </Button>
        </AlertDialogFooter>
      </form>
    </AlertDialogContent>
  </AlertDialog>
</template>

<script setup lang="ts">
import { computed, ref, useId, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { isAxiosError } from 'axios'
import {
  AlertDialog,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from '@/components/ui/alert-dialog'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { deleteNetwork } from '@/api/zerotier/controller'
import { copyToClipboard } from '@/utils/copyToClipboard'
import { showSnackBar } from '@/utils/showSnackBar'

const props = defineProps<{ networkId: string }>()
const emit = defineEmits<{ deleted: [] }>()
const { t } = useI18n()
const idInputId = useId()
const passwordInputId = useId()
const open = ref(false)
const deleting = ref(false)
const confirmationId = ref('')
const password = ref('')
const errorMessage = ref('')
const idMatches = computed(
  () => confirmationId.value.trim().toLowerCase() === props.networkId.toLowerCase(),
)
const canDelete = computed(() => idMatches.value && password.value.length > 0 && !deleting.value)

const updateOpen = (value: boolean) => {
  if (!deleting.value) open.value = value
}

const onEscape = (event: KeyboardEvent) => {
  if (deleting.value) event.preventDefault()
}

watch(open, () => {
  confirmationId.value = ''
  password.value = ''
  errorMessage.value = ''
})

const confirmDelete = async () => {
  if (!canDelete.value) return

  deleting.value = true
  errorMessage.value = ''
  try {
    await deleteNetwork(props.networkId, password.value, confirmationId.value)
    open.value = false
    showSnackBar(t('network.delete.success'), 'success')
    emit('deleted')
  } catch (error: unknown) {
    const status = isAxiosError(error) ? error.response?.status : undefined
    errorMessage.value = t(
      status === 403
        ? 'network.delete.wrongPassword'
        : status === 429
          ? 'network.delete.tooManyAttempts'
          : 'network.delete.failed',
    )
  } finally {
    password.value = ''
    deleting.value = false
  }
}
</script>
