<template>
  <div class="app-wrapper" :class="classObj">
    <div v-if="device === 'mobile' && !app.isCollapse" class="drawer-bg" @click.prevent="handleClickOutside" />
    <!-- 侧边栏 -->
    <side-bar class="sidebar-container" />
    <div :class="{ hasTagsView: needTagView }" class="main-container">
      <div :class="{ 'fixed-header': fixedHeader }">
        <!-- 导航栏 -->
        <nav-bar @setLayout="setLayout" />
        <!-- 历史标签栏 -->
        <tag-view v-if="needTagView" />
      </div>
      <app-main/>
      <!-- 设置 -->
      <Settings ref="settingRef"/>
    </div>
  </div>
</template>

<script setup lang="ts">
import useStore from "@/stores";
import { computed, ref } from "vue";
import SideBar from "./components/SideBar/index.vue";
import NavBar from "./components/NavBar/index.vue";
import AppMain from "./components/AppMain/index.vue";
import TagView from "@/components/TagView/index.vue";
import Settings from "@/components/Settings/index.vue"

const { app, setting } = useStore();
const settingRef = ref();
const device = computed(() => app.device);
const classObj = computed(() => ({
  hideSidebar: app.isCollapse,
  openSidebar: !app.isCollapse,
  mobile: device.value === "mobile",
}));
const fixedHeader = computed(() => setting.fixedHeader);
const needTagView = computed(() => setting.tagView);
const handleClickOutside = () => {
  app.changeCollapse(true);
};
const setLayout = () => {
  settingRef.value.openSetting();
};
</script>

<style scoped lang="scss">
@use "@/assets/styles/mixin" as *;
@use "@/assets/styles/variables.module" as *;

.app-wrapper {
  @include clearfix;
  position: relative;
  height: 100%;
  width: 100%;

  &.mobile.openSidebar {
    position: fixed;
    top: 0;
  }
}

.drawer-bg {
  background: #000;
  opacity: 0.3;
  width: 100%;
  top: 0;
  height: 100%;
  position: absolute;
  z-index: 999;
}

.fixed-header {
  position: fixed;
  top: 0;
  right: 0;
  z-index: 40;
  width: calc(100% - #{$sideBarWidth});
  transition: width 0.28s;
}

.hideSidebar .fixed-header {
  width: calc(100% - 64px);
}

.sidebarHide .fixed-header {
  width: 100%;
}

.mobile .fixed-header {
  width: 100%;
}
</style>
