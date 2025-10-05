import pluginVue from 'eslint-plugin-vue'
import vueTsEslintConfig from '@vue/eslint-config-typescript'
import oxlint from 'eslint-plugin-oxlint'
import skipFormatting from '@vue/eslint-config-prettier/skip-formatting'

export default [
  {
    name: 'app/files-to-ignore',
    ignores: ['src/components/networkDetail/overView/popupPanel/rulesCompile.ts'],
  },

  {
    name: 'app/files-to-lint',
    files: ['**/*.{ts,mts,tsx,vue}'],
  },

  {
    name: 'app/files-to-ignore',
    ignores: [
      '**/dist/**',
      '**/dist-ssr/**',
      '**/coverage/**',
      'src/components/networkDetail/overView/popupPanel/rulesCompile.ts',
    ],
  },

  ...pluginVue.configs['flat/essential'],
  ...vueTsEslintConfig(),
  oxlint.configs['flat/recommended'],
  skipFormatting,

  // UI component libraries allow for single-word naming.
  {
    name: 'app/ui-components-naming',
    files: ['src/components/ui/**/*.vue'],
    rules: {
      'vue/multi-word-component-names': 'off',
    },
  },

  // Oxlint rules exceptions
  {
    name: 'app/oxlint-exceptions',
    files: ['src/components/ui/sidebar/SidebarProvider.vue', 'src/stores/auth.ts'],
    rules: {
      'no-unused-vars': 'off',
    },
  },
]
