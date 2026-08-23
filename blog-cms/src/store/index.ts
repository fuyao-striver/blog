import useAppStore from "./modules/app";
import useSettingStore from "./modules/setting";
import useUserStore from "./modules/user";

const useStore = () => ({
  user: useUserStore(),
  app: useAppStore(),
  setting: useSettingStore(),
});

export default useStore;
