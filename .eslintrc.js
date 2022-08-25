module.exports = {
  root: true,
  env: {
    node: true,
    // Does not work well with <script setup>
    // https://eslint.vuejs.org/user-guide/#faq
    'vue/setup-compiler-macros': true,
  },
  extends: [
    "plugin:vue/vue3-essential",
    "eslint:recommended",
    "@vue/typescript"
  ],
  parserOptions: {
    parser: "@typescript-eslint/parser"
  },
  rules: {}
}
