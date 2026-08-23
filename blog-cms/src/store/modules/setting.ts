import { defineStore } from "pinia";
import type { SettingState } from "../interface";
import defaultSettings from "@/settings";
const { tagView, fixedHeader, sidebarLogo } = defaultSettings;

const useSettingStore = defineStore("useSettingStore", {
  state: (): SettingState => ({
    tagView: tagView,
    fixedHeader: fixedHeader,
    sidebarLogo: sidebarLogo,
  }),
});

export default useSettingStore;
