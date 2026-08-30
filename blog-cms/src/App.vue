<script setup lang="ts">
import zhCn from "element-plus/es/locale/lang/zh-cn";
import useStore from "./store";
import { computed } from "vue";
import type { EpPropMergeType } from "element-plus/es/utils/index.mjs";
const { app } = useStore();
const size = computed(() => app.size as EpPropMergeType<StringConstructor, "default" | "small" | "large", never>);
</script>

<template>
  <el-config-provider :locale="zhCn" :size="size">
    <router-view />
  </el-config-provider>
</template>

<style scoped></style>
