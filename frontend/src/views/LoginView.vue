<script setup lang="ts">
import { login } from '@/api/manage/auth'
import { showSnackBar } from '@/utils/showSnackBar'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'

const { t } = useI18n()
const router = useRouter()

const Login = (e: Event) => {
  // Prevent the default form submission
  e.preventDefault()

  // Get the form data
  const formData = new FormData(e.target as HTMLFormElement)

  // Extract username and password
  const username = formData.get('username') as string
  const password = formData.get('password') as string

  // Call the login function from the API
  login({ username, password })
    .then(() => {
      router.push({ name: 'networks' })
    })
    .catch((_) => {
      showSnackBar(t('auth.loginFailed'), 'error')
    })
}
</script>

<template>
  <div class="flex min-h-full flex-1 flex-col justify-center px-6 py-12 lg:px-8">
    <!-- Logo -->
    <div class="sm:mx-auto sm:w-full sm:max-w-sm">
      <img class="mx-auto h-30 w-auto" src="/icon.png" alt="ZTVRUI Icon" />
    </div>

    <!-- Form container -->
    <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
      <form class="space-y-6" @submit="Login">
        <div>
          <label for="email" class="block text-sm/6 font-medium text-gray-900">{{
            t('auth.username')
          }}</label>
          <div class="mt-2">
            <input
              type="username"
              name="username"
              id="username"
              autocomplete="username"
              class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6"
            />
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between">
            <label for="password" class="block text-sm/6 font-medium text-gray-900">{{
              t('auth.password')
            }}</label>
            <div class="text-sm">
              <a href="#" class="font-semibold text-indigo-600 hover:text-indigo-500">{{
                t('auth.forgotPassword')
              }}</a>
            </div>
          </div>
          <div class="mt-2">
            <input
              type="password"
              name="password"
              id="password"
              autocomplete="current-password"
              class="block w-full rounded-md bg-white px-3 py-1.5 text-base text-gray-900 outline-1 -outline-offset-1 outline-gray-300 placeholder:text-gray-400 focus:outline-2 focus:-outline-offset-2 focus:outline-indigo-600 sm:text-sm/6"
            />
          </div>
        </div>

        <div>
          <button
            type="submit"
            class="flex w-full justify-center rounded-md bg-indigo-600 px-3 py-1.5 text-sm/6 font-semibold text-white shadow-xs hover:bg-indigo-500 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
          >
            {{ t('auth.login') }}
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<style>
@import 'tailwindcss';
</style>
