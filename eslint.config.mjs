import js from '@eslint/js';
import { defineConfig } from 'eslint/config';
import prettier from 'eslint-config-prettier';
import importPlugin from 'eslint-plugin-import';
import pluginVue from 'eslint-plugin-vue';
import globals from 'globals';
import tseslint from 'typescript-eslint';

export default defineConfig([
  importPlugin.flatConfigs.recommended,
  {
    ignores: ['node_modules', 'dist', 'public', '.DS_Store'],
  },
  tseslint.configs.recommended,
  {
    files: ['**/*.{js,mjs,cjs,ts,vue,tsx}'],
    plugins: { js },
    extends: ['js/recommended'],
    rules: {
      '@typescript-eslint/no-require-imports': 'off',
      '@typescript-eslint/ban-ts-comment': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unused-vars': 'warn',
      '@typescript-eslint/no-unused-expressions': 'off',

      'no-unused-vars': 'off',
      'import/named': 'off',
      'import/no-named-as-default': 'off',
      'import/no-unresolved': 'off',
      'import/no-duplicates': 'error',
      'import/newline-after-import': ['error', { count: 1 }],
      'import/order': [
        'error',
        {
          'newlines-between': 'always',
          groups: ['builtin', 'external', 'internal', 'parent', 'sibling', 'index'],
          pathGroups: [
            {
              pattern: '@/**',
              group: 'internal',
              position: 'after',
            },
          ],
          alphabetize: {
            order: 'asc',
            caseInsensitive: true,
          },
        },
      ],
      'sort-imports': [
        'error',
        {
          ignoreCase: true,
          ignoreDeclarationSort: true,
          ignoreMemberSort: false,
          memberSyntaxSortOrder: ['none', 'all', 'multiple', 'single'],
        },
      ],
    },
  },
  {
    // files: ['**/*.{js,mjs,cjs,ts,vue}'],
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.node,
      },
    },
  },
  pluginVue.configs['flat/essential'],
  prettier,
  {
    files: ['**/*.vue'],
    languageOptions: { parserOptions: { parser: tseslint.parser } },
    rules: {
      'vue/attributes-order': [
        'error',
        {
          order: [
            'DEFINITION',
            'LIST_RENDERING',
            'CONDITIONALS',
            'RENDER_MODIFIERS',
            'GLOBAL',
            ['UNIQUE', 'SLOT'],
            'TWO_WAY_BINDING',
            'OTHER_DIRECTIVES',
            'OTHER_ATTR',
            'EVENTS',
            'CONTENT',
          ],
          alphabetical: false,
        },
      ],
      'vue/no-undef-properties': 'error',
    },
  },
  {
    rules: {
      'vue/multi-word-component-names': 'off',
    },
  },
]);
