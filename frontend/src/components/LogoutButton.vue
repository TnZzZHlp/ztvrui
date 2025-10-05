<template>
    <button @click="handleLogout"
        class="flex items-center px-4 py-2 text-sm text-gray-700 hover:bg-gray-100 hover:text-gray-900" role="menuitem">
        <svg class="mr-3 h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd"
                d="M3 3a1 1 0 00-1 1v12a1 1 0 102 0V4a1 1 0 00-1-1zm10.293 9.293a1 1 0 001.414 1.414l3-3a1 1 0 000-1.414l-3-3a1 1 0 10-1.414 1.414L14.586 9H7a1 1 0 100 2h7.586l-1.293 1.293z"
                clip-rule="evenodd" />
        </svg>
        {{ t('auth.logout') }}
    </button>
</template>

<script setup lang="ts">
import { logout } from '@/api/manage/auth'
import { showSnackBar } from '@/utils/showSnackBar'
import { useAuthStore } from '@/stores/auth'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const { t } = useI18n()
const router = useRouter()
const authStore = useAuthStore()

const handleLogout = async () => {
    try {
        await logout()
        showSnackBar(t('auth.logout') + ' ' + t('common.updateSuccess'), 'success')
    } catch (error) {
        console.warn('Logout API call failed:', error)
        // Still proceed with local logout even if server call fails
    } finally {
        // Always clear local session and redirect
        router.push({ name: 'login' })
    }
}
</script>
