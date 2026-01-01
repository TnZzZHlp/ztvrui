<script setup lang="ts">
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useThemeStore } from '@/stores/theme'
import { Button } from '@/components/ui/button'
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from '@/components/ui/tooltip'
import { Moon, Sun } from 'lucide-vue-next'

const { t } = useI18n()
const themeStore = useThemeStore()

const isDark = computed(() => themeStore.theme === 'dark')
</script>

<template>
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger as-child>
        <Button
          variant="ghost"
          size="icon"
          class="h-9 w-9"
          @click="themeStore.toggleTheme"
        >
          <Sun
            v-if="!isDark"
            class="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0"
          />
          <Moon
            v-else
            class="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100"
          />
          <span class="sr-only">{{ t('common.themeToggle') }}</span>
        </Button>
      </TooltipTrigger>
      <TooltipContent>
        <p>{{ t(isDark ? 'common.lightMode' : 'common.darkMode') }}</p>
      </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>
