module.exports = {
  env: {
    browser: true,
    es2021: true,
    node: true
  },
  extends: [
    'plugin:vue/essential',
    'plugin:vue/vue3-recommended'
  ],
  parserOptions: {
    ecmaVersion: 'latest',
    sourceType: 'module'
  },
  plugins: [
    'vue'
  ],
  rules: {
    'semi': [2, 'never'],
    'comma-dangle': [2, 'never'],
    'comma-spacing': [2, {
      'after': true,
      'before': false
    }],
    'comma-style': [2, 'last'],
    'vue/max-attributes-per-line': ['error', {
      'singleline': {
        'max': 5
      },
      'multiline': {
        'max': 5
      }
    }],
    'vue/multi-word-component-names': 'off',
    'vue/singleline-html-element-content-newline': 'off'
  },
  overrides: [
  ]
}
