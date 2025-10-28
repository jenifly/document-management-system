<template>
  <div class="shared-view">
    <n-card title="我的分享">
      <n-empty description="此功能正在开发中，敬请期待">
        <template #extra>
          <n-button @click="router.push('/documents')">
            返回文档列表
          </n-button>
        </template>
      </n-empty>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'
import { NCard, NEmpty, NButton } from 'naive-ui'

const router = useRouter()
</script>

<style scoped>
.shared-view {
  max-width: 1400px;
  margin: 0 auto;
}
</style>
