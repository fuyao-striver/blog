import useAppStore from "./modules/app";
import usePermissionStore from "./modules/permission";
import useSettingStore from "./modules/setting";
import useTagStore from "./modules/tag";
import useUserStore from "./modules/user";

const useStore = () => ({
  app: useAppStore(),
  user: useUserStore(),
  setting: useSettingStore(),
  tag: useTagStore(),
  permission: usePermissionStore(),
});

export default useStore;
