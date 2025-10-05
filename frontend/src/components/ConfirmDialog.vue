<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from '@/components/ui/alert-dialog'

const { t } = useI18n()

interface Props {
    open: boolean
    message: string
    type?: 'normal' | 'warn'
    onConfirm: () => void
}

const props = withDefaults(defineProps<Props>(), {
    type: 'normal',
})

const emit = defineEmits<{
    'update:open': [value: boolean]
}>()

const handleConfirm = () => {
    props.onConfirm()
    emit('update:open', false)
}

const handleCancel = () => {
    emit('update:open', false)
}
</script>

<template>
    <AlertDialog :open="open" @update:open="(val) => emit('update:open', val)">
        <AlertDialogContent>
            <AlertDialogHeader>
                <AlertDialogTitle>{{ message }}</AlertDialogTitle>
                <AlertDialogDescription v-if="type === 'warn'">
                    {{ t('common.confirmAction') }}
                </AlertDialogDescription>
            </AlertDialogHeader>
            <AlertDialogFooter>
                <AlertDialogCancel @click="handleCancel">
                    {{ t('common.cancel') }}
                </AlertDialogCancel>
                <AlertDialogAction @click="handleConfirm" :class="{
                    'bg-red-500 hover:bg-red-600': type === 'warn',
                    'bg-blue-500 hover:bg-blue-600': type === 'normal',
                }">
                    {{ t('common.confirm') }}
                </AlertDialogAction>
            </AlertDialogFooter>
        </AlertDialogContent>
    </AlertDialog>
</template>
