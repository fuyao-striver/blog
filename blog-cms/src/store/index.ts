import useAppStore from "./modules/app";
import usePermissionStore from "./modules/permission";
import useSettingStore from "./modules/setting";
import useTagStore from "./modules/tag";
import useUserStore from "./modules/user";

const useStore = () => ({
  user: useUserStore(),
  app: useAppStore(),
  setting: useSettingStore(),
  permission: usePermissionStore(),
  tag: useTagStore(),
});

export default useStore;
