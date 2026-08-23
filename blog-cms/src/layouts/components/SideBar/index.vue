<template>
  <div :class="{ 'has-logo': showLogo }">
    <!--网站logo-->
    <logo v-if="showLogo" :collapse="isCollapse" />
    <!--侧边栏-->
    <el-scrollbar wrap-class="scrollbar-wrapper">
      <el-menu></el-menu>
    </el-scrollbar>
  </div>
</template>

<script setup lang="ts">
import useStore from "@/store";
import { computed } from "vue";
import Logo from "./Logo.vue";

const { app, setting } = useStore();
const showLogo = computed(() => setting.sidebarLogo);
const isCollapse = computed(() => app.isCollapse);
</script>
