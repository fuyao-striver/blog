<template>
  <div class="top-right-btn" :style="style">
    <el-row>
      <el-tooltip class="item" effect="dark" :content="showSearch ? '隐藏搜索' : '显示搜索'" placement="top" v-if="search">
        <el-button circle @click.prevent="toggleSearch()">
          <template #icon>
            <svg-icon icon-class="search"></svg-icon>
          </template>
        </el-button>
      </el-tooltip>
      <el-tooltip class="item" effect="dark" content="刷新" placement="top">
        <el-button circle @click.prevent="refresh()">
          <template #icon>
            <svg-icon icon-class="refresh"></svg-icon>
          </template>
        </el-button>
      </el-tooltip>
      <el-tooltip class="item" effect="dark" content="显隐列" placement="top" v-if="columns">
        <el-button circle @click.prevent="showColumn()">
          <template #icon>
            <svg-icon icon-class="menu"></svg-icon>
          </template>
        </el-button>
      </el-tooltip>
    </el-row>
    <el-dialog :title="title" v-model="open" append-to-body>
      <el-transfer :titles="['显示', '隐藏']" v-model="value" :date="columns" @change="dataChange"></el-transfer>
    </el-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";

const open = ref(false);
const value = ref([]);
const title = ref("显示/隐藏");
const props = defineProps({
  showSearch: {
    type: Boolean,
    default: true,
  },
  columns: {
    type: Array,
  },
  search: {
    type: Boolean,
    default: true,
  },
  gutter: {
    type: Number,
    default: 10,
  },
});

const dataChange = (data) => {
  for (let item in props.columns) {
    const key = props.columns[item].key;
    props.columns[item].visible = !data.includes(key);
  }
};

const style = computed(() => {
  const ret = {};
  if (props.gutter) {
    ret.marginRight = `${props.gutter / 2}px`;
  }
  return ret;
});
const emits = defineEmits(["update:showSearch", "queryTable"]);
const toggleSearch = () => {
  emits("update:showSearch", !props.showSearch);
};
const refresh = () => {
  emits("queryTable");
};
const showColumn = () => {
  open.value = true;
};
</script>

<style lang="scss" scoped>
:deep(.el-transfer__button) {
  border-radius: 50%;
  display: block;
  margin-left: 0px;
}

:deep(.el-transfer__button:first-child) {
  margin-bottom: 10px;
}

.my-el-transfer {
  text-align: center;
}
</style>
