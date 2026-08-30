<template>
  <div class="app-container">
    <el-tabs type="border-card" class="demo-tabs">
      <!-- 网站信息 -->
      <el-tab-pane>
        <template #label>
          <span class="custom-tabs-label">
            <svg-icon icon-class="platform" />
            <span>网站信息</span>
          </span>
        </template>
        <el-form label-width="80px" :model="siteConfig" label-position="left">
          <el-row>
            <el-col :md="6">
              <el-form-item label="用户头像">
                <el-upload
                  class="avatar-uploader"
                  :headers="authorization"
                  achtion="/api/admin/site/upload"
                  :show-file-list="false"
                  accept="image/*"
                  :before-upload="beforeUpload"
                  :on-success="handleArticleSuccess"
                >
                  <img v-if="siteConfig.articleCover" :src="siteConfig.articleCover" class="article-cover" />
                  <svg-icon v-else class="avatar-uploader-icon" icon-class="plus"></svg-icon>
                </el-upload>
              </el-form-item>
            </el-col>
            <el-col :md="6">
              <el-form-item label="游客头像">
                <el-upload
                  class="avatar-uploader"
                  :headers="authorization"
                  action="/api/admin/site/upload"
                  :show-file-list="false"
                  accept="image/*"
                  :before-upload="beforeUpload"
                  :on-success="handleTouristAvatarSuccess"
                >
                  <img v-if="siteConfig.touristAvatar" :src="siteConfig.touristAvatar" class="avatar" />
                  <svg-icon v-else class="avatar-uploader-icon" icon-class="plus"></svg-icon>
                </el-upload>
              </el-form-item>
            </el-col>
          </el-row>
          <el-form-item label="网站名称">
            <el-input v-model="siteConfig.siteName" style="width: 400px"></el-input>
          </el-form-item>
          <el-form-item label="网站地址">
            <el-input v-model="siteConfig.siteAddress" style="width: 400px"></el-input>
          </el-form-item>
          <el-form-item label="网站简介">
            <el-input v-model="siteConfig.siteIntro" style="width: 400px"></el-input>
          </el-form-item>
          <el-form-item label="网站公告">
            <el-input
              style="width: 400px"
              v-model="siteConfig.siteNotice"
              :autosize="{ minRows: 4, maxRows: 5 }"
              resize="none"
              type="textarea"
            ></el-input>
          </el-form-item>
          <el-form-item label="建站日期">
            <el-date-picker v-model="siteConfig.createSiteTime" value-format="YYYY-MM-DD" type="date" placeholder="选择日期"></el-date-picker>
          </el-form-item>
          <el-form-item label="备案号">
            <el-input v-model="siteConfig.recordNumber" style="width: 400px"></el-input>
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click.prevent="handleUpdate">保 存</el-button>
          </el-form-item>
        </el-form>
      </el-tab-pane>
      <!-- 作者信息 -->
      <el-tab-pane label="author">
        <template #label>
          <svg-icon class="custom-tabs-label" icon-class="flag" />
          <span>作者信息</span>
        </template>
        <el-form label-width="80px" :model="siteConfig" label-position="left">
          <el-form-item label="作者头像">
            <el-upload
              class="avatar-uploader"
              :headers="authorization"
              action="/api/admin/site/upload"
              :show-file-list="false"
              accept="image/*"
              :before-upload="beforeUpload"
              :on-success="handleAuthorAvatarSuccess"
            >
              <img v-if="siteConfig.authorAvatar" :src="siteConfig.authorAvatar" class="avatar" alt="avatar" />
              <svg-icon v-else class="avatar-uploader-icon" icon-class="plus" />
            </el-upload>
          </el-form-item>
          <el-form-item label="网站作者">
            <el-input v-model="siteConfig.siteAuthor" style="width: 400px" />
          </el-form-item>
          <el-form-item label="关于我">
            <!--md编辑器-->
            <md-editor v-model="siteConfig.aboutMe" height="400px" />
          </el-form-item>
          <el-form-item>
            <el-button type="primary" @click.prevent="handleUpdate">保 存</el-button>
          </el-form-item>
        </el-form>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script lang="ts" setup>
import type { SiteConfig } from "@/api/site/types";
import { getToken, token_prefix } from "@/utils/token";
import type { AxiosResponse } from "axios";
import type { UploadRawFile } from "element-plus";
import { computed, onMounted, ref } from "vue";
import * as imageConversion from "image-conversion";
import { getSiteConfig } from "@/api/site";

const siteConfig = ref<SiteConfig>({} as SiteConfig);
const authorization = computed(() => ({
  Authorization: token_prefix + getToken(),
}));

const handleArticleSuccess = (response: AxiosResponse) => {
  siteConfig.value.articleCover = response.data;
};

const handleAuthorAvatarSuccess = (response: AxiosResponse) => {
  siteConfig.value.authorAvatar = response.data;
};
const beforeUpload = (rawFile: UploadRawFile) => {
  return new Promise((resolve) => {
    if (rawFile.size / 1024 < 200) {
      resolve(rawFile);
    }
    // 压缩到200KB,这里的200就是要压缩的大小,可自定义
    imageConversion.compressAccurately(rawFile, 200).then((res) => {
      resolve(res);
    });
  });
};
const handleTouristAvatarSuccess = (response: AxiosResponse) => {
  siteConfig.value.touristAvatar = response.data;
};
const handleUpdate = () => {
  // todo
};

const getList = () => {
  getSiteConfig().then(({ data }) => {
    siteConfig.value = data.data;
  });
};

onMounted(() => {
  getList();
});
</script>

<style lang="scss" scoped>
.demo-tabs > .el-tabs__content {
  padding: 32px;
  color: #6b778c;
  font-size: 32px;
  font-weight: 600;
}

.demo-tabs .custom-tabs-label .el-icon {
  vertical-align: middle;
}

.demo-tabs .custom-tabs-label span {
  vertical-align: middle;
  margin-left: 4px;
}

.avatar-uploader .avatar {
  width: 120px;
  height: 120px;
  object-fit: contain;
}

.article-cover {
  width: 300px;
}
</style>
