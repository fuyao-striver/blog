import useUserStore from "@/stores/modules/user.ts";

const useStore = () => ({
  user: useUserStore(),
});

export default useStore;
