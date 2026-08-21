import { defineStore } from "pinia";
import type { UserState } from "../interface";
import type { LoginForm } from "@/api/login/type";
import { setToken } from "@/utils/token";
import { login } from "@/api/login";

const useUserStore = defineStore("useUserStore", {
  state: (): UserState => ({
    id: null,
    avatar: "",
    roleList: [],
    permissionList: [],
  }),
  actions: {
    LogIn(LoginForm: LoginForm) {
      return new Promise((resolve, reject) => {
        login(LoginForm)
          .then(({ data }) => {
            if (data.flag) {
              setToken(data.data);
              resolve(data);
            } else {
              reject(data.msg);
            }
          })
          .catch((error) => {
            reject(error);
          });
      });
    },
  },
});

export default useUserStore;
