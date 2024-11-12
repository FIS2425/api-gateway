import globals from 'globals';
import pluginJs from '@eslint/js';
import stylisticJs from '@stylistic/eslint-plugin-js';

export default [
  {
    plugins: {
      '@stylistic/js': stylisticJs,
    },
    rules: {
      'no-empty': ['error', { allowEmptyCatch: true }],
      '@stylistic/js/quotes': ['error', 'single', { avoidEscape: true }],
      '@stylistic/js/linebreak-style': ['error', 'unix'],
      '@stylistic/js/indent': ['error', 2],
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        process: 'readonly',
      },
    },
  },
  pluginJs.configs.recommended,
];
