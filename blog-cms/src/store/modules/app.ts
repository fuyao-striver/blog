import { defineStore } from "pinia";
import type { AppState } from "../interface";

const useAppStore = defineStore("useAppStore", {
  state: (): AppState => ({
    isCollapse: false,
    device: "desktop",
    size: "dafault",
  }),
  actions: {
    toggle() {
      this.isCollapse = !this.isCollapse;
    },
    changeCollapse(isCollapse: boolean) {
      this.isCollapse = isCollapse;
    },
    toggleDevice(device: string) {
      this.device = device;
    },
    setSize(size: string) {
      this.size = size;
    },
  },
});

export default useAppStore;
