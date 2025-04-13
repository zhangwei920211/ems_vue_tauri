<script setup lang="ts">
defineOptions({
  name: "Welcome"
});
</script>

<template>
  <div class="welcome-container">
    <h1>Pure-Admin-Thin（非国际化版本）</h1>
    <p class="description">
      这是一个基于 Vue3、Vite、Element-Plus、TypeScript
      构建的中后台管理系统模板，现已集成 Tauri v2 支持。
    </p>
  </div>
</template>

<style lang="scss" scoped>
.welcome-container {
  padding: 20px;

  .description {
    margin: 20px 0;
    font-size: 16px;
    color: #666;
  }
}
</style>
