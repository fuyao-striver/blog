<template>
  <div class="app-container">
    <!-- 搜索栏 -->
    <el-form :inline="true" v-show="showSearch" @submit.native.prevent :model="queryParams">
      <el-form-item label="友链名称">
        <el-input @keyup.enter="handleQuery()" v-model="queryParams.keyword" style="width: 200px" placeholder="请输入友链名称" clearable />
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click.prevent="handleQuery()">
          <template #icon>
            <svg-icon icon-class="search" />
          </template>
          搜索
        </el-button>
      </el-form-item>
    </el-form>
    <!-- 操作按钮 -->
    <el-row :gutter="10" class="mb15">
      <el-col :span="1.5">
        <el-button type="primary" plain @click.prevent="openModel(undefined)">
          <template #icon>
            <svg-icon icon-class="plus" />
          </template>
          新增
        </el-button>
      </el-col>
      <el-col :span="1.5">
        <el-button type="danger" plain :disabled="friendIdList.length === 0" @click.prevent="handleDelete(undefined)">
          <template #icon>
            <svg-icon icon-class="delete"></svg-icon>
          </template>
          批量删除
        </el-button>
      </el-col>
      <right-toolbar v-model:showSearch="showSearch" @queryTable="getList()"></right-toolbar>
    </el-row>
  </div>
</template>

<script lang="ts" setup>
import type { Friend, FriendForm, FriendQuery } from "@/api/friend/type";
import { messageConfirm } from "@/utils/modal";
import type { FormInstance } from "element-plus";
import { reactive, ref } from "vue";

const showSearch = ref(true);
const addOrUpdate = ref(false);
const title = ref("");
const friendIdList = ref([]);
const friendForm = ref<FriendForm>({
  id: undefined,
  color: "#409EFF",
  name: "",
  avatar: "",
  url: "",
  introduction: "",
});
const friendFormRef = ref<FormInstance>();
const queryParams = reactive<FriendQuery>({} as FriendQuery);

const openModel = (friend?: Friend) => {
  friendFormRef.value?.clearValidate();
  if (friend !== undefined) {
    friendForm.value = JSON.parse(JSON.stringify(friend));
    title.value = "修改友链";
  } else {
    title.value = "添加友链";
    friendForm.value = {
      id: undefined,
      color: "#409EFF",
      name: "",
      avatar: "",
      url: "",
      introduction: "",
    };
  }
  addOrUpdate.value = true;
};

const handleDelete = (id?: number) => {
  let ids = [];
  if (id === undefined) {
    ids = friendIdList.value;
  } else {
    ids = [id];
  }
  messageConfirm("确定要删除已选中的数据项?").then(() => {
    //todo
  });
};

const getList = () => {
  //todo
};

const handleQuery = () => {
  queryParams.current = 1;
  // todo
};
</script>
